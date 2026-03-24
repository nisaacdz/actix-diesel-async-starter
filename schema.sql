-- Foundation: Extensions and helper functions

CREATE EXTENSION IF NOT EXISTS "pg_uuidv7";
CREATE EXTENSION IF NOT EXISTS "btree_gist";
CREATE EXTENSION IF NOT EXISTS "postgis";

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- Users and Files (with file_type enum)

CREATE TYPE file_type AS ENUM ('image', 'video', 'audio', 'document', 'other');

CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    original_name TEXT,
    label TEXT,
    name TEXT NOT NULL,
    bucket TEXT NOT NULL,
    file_type file_type NOT NULL,
    mime VARCHAR(50) NOT NULL,
    size BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON files
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    phone VARCHAR(15) NOT NULL UNIQUE,
    email VARCHAR(55) UNIQUE,
    full_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- Stops and vehicles

CREATE TYPE vehicle_type AS ENUM ('intercity/bus', 'trotro/metro');

CREATE TABLE stops (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name VARCHAR(255) NOT NULL,
    geom GEOGRAPHY(Point, 4326) NOT NULL,
    is_terminal BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON stops
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE INDEX idx_stops_geom ON stops USING GIST (geom);
CREATE INDEX idx_stops_is_terminal ON stops (is_terminal) WHERE is_terminal = TRUE;

CREATE TABLE vehicles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    license_plate VARCHAR(20) NOT NULL UNIQUE,
    capacity INT NOT NULL,
    v_type vehicle_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON vehicles
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE INDEX idx_vehicles_v_type ON vehicles (v_type);
-- Operators: operators, staff, and vehicle assignments (with operator_staff_role enum)

CREATE TYPE operator_staff_role AS ENUM ('conductor', 'driver', 'admin');

CREATE TABLE operators (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    license_number VARCHAR(255),
    website VARCHAR(255),
    logo_id UUID REFERENCES files(id),
    cover_photo_id UUID REFERENCES files(id),
    contact_phone VARCHAR(20) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    policy_document TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON operators
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE INDEX idx_operators_is_active ON operators (is_active) WHERE is_active = TRUE;

CREATE TABLE operator_staff (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    operator_id UUID NOT NULL REFERENCES operators(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role operator_staff_role NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (operator_id, user_id)
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON operator_staff
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE INDEX idx_operator_staff_user_id ON operator_staff (user_id);
CREATE INDEX idx_operator_staff_role ON operator_staff (operator_id, role);

CREATE TABLE operator_vehicles (
    operator_id UUID NOT NULL REFERENCES operators(id) ON DELETE CASCADE,
    vehicle_id UUID NOT NULL REFERENCES vehicles(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (operator_id, vehicle_id)
);
CREATE INDEX idx_operator_vehicles_vehicle_id ON operator_vehicles (vehicle_id);
-- OTPs (One-Time Passwords)

CREATE TABLE otps (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    phone VARCHAR(255) NOT NULL UNIQUE,
    code VARCHAR(12) NOT NULL,
    payload JSONB,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON otps
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- Routes, Schedules, and Seats (with bus_schedule_status enum)

CREATE TYPE bus_schedule_status AS ENUM ('scheduled', 'boarding', 'in_transit', 'completed', 'cancelled');

CREATE TABLE routes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON routes
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();

CREATE TABLE route_stops (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    route_id UUID NOT NULL REFERENCES routes(id) ON DELETE CASCADE,
    stop_id UUID NOT NULL REFERENCES stops(id),
    stop_order INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (route_id, stop_order)
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON route_stops
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE INDEX idx_route_stops_stop_id ON route_stops (stop_id);

CREATE TABLE bus_schedules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),

    origin_terminal_id UUID NOT NULL REFERENCES stops(id),
    dest_terminal_id UUID NOT NULL REFERENCES stops(id),

    route_id UUID NOT NULL REFERENCES routes(id),
    vehicle_id UUID NOT NULL REFERENCES vehicles(id),
    operator_id UUID NOT NULL REFERENCES operators(id),

    driver_id UUID REFERENCES users(id),

    remaining_seats INT NOT NULL,

    -- MATCH SIMPLE: Succeeds if driver_id is NULL. If present, ensures driver belongs to operator.
    FOREIGN KEY (operator_id, driver_id) REFERENCES operator_staff(operator_id, user_id),

    scheduled_departure TIMESTAMPTZ NOT NULL,
    base_price_pesewas BIGINT NOT NULL,
    discount_amount_pesewas BIGINT NOT NULL DEFAULT 0,

    status bus_schedule_status DEFAULT 'scheduled',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CHECK (discount_amount_pesewas >= 0),
    CHECK (base_price_pesewas > 0),
    CHECK (discount_amount_pesewas < base_price_pesewas)
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON bus_schedules
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE INDEX idx_bus_schedules_operator_id ON bus_schedules (operator_id);
CREATE INDEX idx_bus_schedules_route_id ON bus_schedules (route_id);
CREATE INDEX idx_bus_schedules_status ON bus_schedules (status);
CREATE INDEX idx_bus_schedules_departure ON bus_schedules (scheduled_departure);
CREATE INDEX idx_bus_schedules_origin_dest ON bus_schedules (origin_terminal_id, dest_terminal_id);

-- DB CONSTRAINT: Enforce that scheduled origins/destinations are actually terminals
CREATE OR REPLACE FUNCTION enforce_is_terminal() RETURNS TRIGGER AS $$
DECLARE
    is_orig_term BOOLEAN;
    is_dest_term BOOLEAN;
BEGIN
    SELECT is_terminal INTO is_orig_term FROM stops WHERE id = NEW.origin_terminal_id;
    SELECT is_terminal INTO is_dest_term FROM stops WHERE id = NEW.dest_terminal_id;
    IF NOT is_orig_term THEN RAISE EXCEPTION 'origin_terminal_id must point to a stop where is_terminal is true'; END IF;
    IF NOT is_dest_term THEN RAISE EXCEPTION 'dest_terminal_id must point to a stop where is_terminal is true'; END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_enforce_terminals BEFORE INSERT OR UPDATE ON bus_schedules
    FOR EACH ROW EXECUTE FUNCTION enforce_is_terminal();

-- DB CONSTRAINT: Enforce that the assigned vehicle is strictly a Bus
CREATE OR REPLACE FUNCTION enforce_bus_schedule_vehicle() RETURNS TRIGGER AS $$
DECLARE
    v_type_val vehicle_type;
BEGIN
    SELECT v_type INTO v_type_val FROM vehicles WHERE id = NEW.vehicle_id;
    IF v_type_val != 'intercity/bus' THEN
        RAISE EXCEPTION 'Vehicle assigned to a bus schedule must be of type intercity/bus';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_enforce_bus_schedule_vehicle BEFORE INSERT OR UPDATE ON bus_schedules
    FOR EACH ROW EXECUTE FUNCTION enforce_bus_schedule_vehicle();

CREATE TABLE bus_schedule_seats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    schedule_id UUID NOT NULL REFERENCES bus_schedules(id) ON DELETE CASCADE,
    seat_number VARCHAR(10) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (schedule_id, seat_number)
);
CREATE INDEX idx_bus_schedule_seats_schedule_id ON bus_schedule_seats (schedule_id);
-- Bookings and Tickets

CREATE TYPE booking_status AS ENUM ('pending', 'completed', 'cancelled');
CREATE TYPE ticket_status AS ENUM ('pending', 'banned', 'ok');

CREATE TABLE bus_bookings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    schedule_id UUID NOT NULL REFERENCES bus_schedules(id),
    user_id UUID NOT NULL REFERENCES users(id),
    ticket_count INT NOT NULL, -- denormalized for speed? maybe remove? we'll see if it's ever used
    total_cost BIGINT NOT NULL, -- sum of ticket costs: currently = bus_schedule.price * ticket_count
    discount_amount BIGINT NOT NULL DEFAULT 0, -- copied from bus_schedules.discount_amount_pesewas
    vat_amount BIGINT NOT NULL, -- inserted from config in configs/default.toml, updated regularly
    payable_amount BIGINT NOT NULL, -- total_cost - discount_amount + vat_amount

    payment_expires_at TIMESTAMPTZ NOT NULL, -- not yet decided on an algorithm for booking to prevent double booking, keep this?

    status booking_status NOT NULL DEFAULT 'pending',

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CHECK (total_cost - discount_amount + vat_amount = payable_amount),
    CHECK (ticket_count > 0),
    CHECK (total_cost > 0),
    CHECK (discount_amount >= 0),
    CHECK (vat_amount >= 0),
    CHECK (payable_amount >= 0),
    CHECK (discount_amount <= total_cost),
    CHECK (vat_amount <= payable_amount)
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON bus_bookings
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE INDEX idx_bus_bookings_schedule_id ON bus_bookings (schedule_id);
CREATE INDEX idx_bus_bookings_user_id ON bus_bookings (user_id);
CREATE INDEX idx_bus_bookings_status ON bus_bookings (status);

CREATE TABLE bus_tickets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    booking_id UUID NOT NULL REFERENCES bus_bookings(id) ON DELETE CASCADE,
    seat_id UUID NOT NULL REFERENCES bus_schedule_seats(id),
    stop_id UUID NOT NULL REFERENCES stops(id),
    guest_name VARCHAR(255),
    guest_phone VARCHAR(255),
    guest_email VARCHAR(255),
    qr_payload TEXT UNIQUE,
    status ticket_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER set_updated_at BEFORE UPDATE ON bus_tickets
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
CREATE INDEX idx_bus_tickets_booking_id ON bus_tickets (booking_id);
CREATE INDEX idx_bus_tickets_seat_id ON bus_tickets (seat_id)
    WHERE status = 'ok';


-- DB CONSTRAINT: Prevent Double Booking (only one active ticket per seat)
CREATE UNIQUE INDEX idx_bus_tickets_unique_seat ON bus_tickets (seat_id);

CREATE OR REPLACE FUNCTION enforce_ticket_schedule_match() RETURNS TRIGGER AS $$
DECLARE
    v_booking_schedule_id UUID;
    v_seat_schedule_id UUID;
BEGIN
    SELECT schedule_id INTO v_booking_schedule_id 
    FROM bus_bookings WHERE id = NEW.booking_id;
    
    SELECT schedule_id INTO v_seat_schedule_id 
    FROM bus_schedule_seats WHERE id = NEW.seat_id;
    
    IF v_booking_schedule_id != v_seat_schedule_id THEN
        RAISE EXCEPTION 'Mismatched schedules: The booked seat does not belong to the schedule of the booking.';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_a_enforce_schedule_match 
BEFORE INSERT OR UPDATE ON bus_tickets
FOR EACH ROW EXECUTE FUNCTION enforce_ticket_schedule_match();

-- DB CONSTRAINT: Enforce stop_id is among the unified route stops OR the dest terminal
CREATE OR REPLACE FUNCTION enforce_ticket_stop() RETURNS TRIGGER AS $$
DECLARE
    v_route_id UUID;
    v_dest_term_id UUID;
    is_valid BOOLEAN;
BEGIN
    SELECT s.route_id, s.dest_terminal_id INTO v_route_id, v_dest_term_id
    FROM bus_bookings b
    JOIN bus_schedules s ON b.schedule_id = s.id
    WHERE b.id = NEW.booking_id;

    SELECT EXISTS (
        SELECT 1 FROM route_stops WHERE route_id = v_route_id AND stop_id = NEW.stop_id
    ) INTO is_valid;

    IF NOT is_valid THEN
        IF NEW.stop_id = v_dest_term_id THEN
            is_valid := TRUE;
        END IF;
    END IF;

    IF NOT is_valid THEN
        RAISE EXCEPTION 'Ticket stop_id must be a valid intermediate route stop or the destination terminal for this schedule';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_enforce_ticket_stop BEFORE INSERT OR UPDATE ON bus_tickets
    FOR EACH ROW EXECUTE FUNCTION enforce_ticket_stop();
-- Payments
CREATE TYPE payment_status AS ENUM ('pending', 'initiated', 'failed', 'verified');
CREATE TYPE payment_provider AS ENUM ('paystack');

CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),

    bus_booking_id UUID REFERENCES bus_bookings(id) ON DELETE CASCADE,
    trotro_pass_id UUID, -- Will reference trotro_passes

    amount_pesewas BIGINT,

    provider payment_provider NOT NULL,
    provider_id TEXT, -- provider's own id of this transaction, set when we get response from them, aka `reference`
    internal_id TEXT NOT NULL, -- generated by us, sent to the provider
    
    status payment_status NOT NULL,

    provider_request_payload JSONB,
    provider_response_payload JSONB,
    provider_callback_payload JSONB,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT chk_payment_target CHECK (
        (bus_booking_id IS NOT NULL AND trotro_pass_id IS NULL) OR 
        (bus_booking_id IS NULL AND trotro_pass_id IS NOT NULL)
    )
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON payments
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
-- Admins table

CREATE TABLE admins (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID UNIQUE NOT NULL REFERENCES users(id),

    -- later to add hierarchical `roles` column
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON admins
    FOR EACH ROW EXECUTE FUNCTION update_timestamp();
