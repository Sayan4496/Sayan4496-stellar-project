#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, Address, Map
};

#[contract]
pub struct TravelBooking;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    BookingCounter,
    Bookings,
}

#[contracttype]
#[derive(Clone)]
pub struct Booking {
    pub id: u32,
    pub user: Address,
    pub destination: Symbol,
    pub date: Symbol,
    pub paid: bool,
}

#[contractimpl]
impl TravelBooking {

    pub fn create_booking(env: Env, user: Address, destination: Symbol, date: Symbol) -> u32 {
        user.require_auth();

        let mut counter: u32 = env
            .storage()
            .instance()
            .get(&DataKey::BookingCounter)
            .unwrap_or(0);

        counter += 1;

        let booking = Booking {
            id: counter,
            user: user.clone(),
            destination,
            date,
            paid: false,
        };

        let mut bookings: Map<u32, Booking> = env
            .storage()
            .instance()
            .get(&DataKey::Bookings)
            .unwrap_or(Map::new(&env));

        bookings.set(counter, booking);

        env.storage().instance().set(&DataKey::Bookings, &bookings);
        env.storage().instance().set(&DataKey::BookingCounter, &counter);

        counter
    }

    pub fn pay_booking(env: Env, user: Address, booking_id: u32) {
        user.require_auth();

        let mut bookings: Map<u32, Booking> =
            env.storage().instance().get(&DataKey::Bookings).unwrap();

        let mut booking = bookings.get(booking_id).unwrap();

        if booking.user != user {
            panic!("Unauthorized");
        }

        booking.paid = true;
        bookings.set(booking_id, booking);

        env.storage().instance().set(&DataKey::Bookings, &bookings);
    }

    pub fn get_booking(env: Env, booking_id: u32) -> Booking {
        let bookings: Map<u32, Booking> =
            env.storage().instance().get(&DataKey::Bookings).unwrap();

        bookings.get(booking_id).unwrap()
    }
}