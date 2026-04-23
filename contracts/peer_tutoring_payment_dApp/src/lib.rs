#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol, Vec
};

#[contract]
pub struct TutorPaymentContract;

#[contracttype]
#[derive(Clone)]
pub struct Payment {
    pub student: Address,
    pub tutor: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    PaymentHistory,                // Vec<Payment>
    TotalPaid(Address, Address),  // (student, tutor) -> amount
}

#[contractimpl]
impl TutorPaymentContract {

    // =========================
    // PAY TUTOR (MAIN FUNCTION)
    // =========================
    pub fn pay_tutor(env: Env, student: Address, tutor: Address, amount: i128) {

        // 🔐 Xác thực người gửi
        student.require_auth();

        // ❌ validate cơ bản
        if amount <= 0 {
            panic!("Amount must be > 0");
        }

        // =========================
        // 1. Cập nhật tổng tiền
        // =========================
        let key = DataKey::TotalPaid(student.clone(), tutor.clone());

        let current: i128 = env.storage()
            .instance()
            .get(&key)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&key, &(current + amount));

        // =========================
        // 2. Lưu lịch sử payment
        // =========================
        let mut history: Vec<Payment> = env.storage()
            .instance()
            .get(&DataKey::PaymentHistory)
            .unwrap_or(Vec::new(&env));

        let payment = Payment {
            student: student.clone(),
            tutor: tutor.clone(),
            amount,
            timestamp: env.ledger().timestamp(),
        };

        history.push_back(payment);

        env.storage()
            .instance()
            .set(&DataKey::PaymentHistory, &history);
    }

    // =========================
    // GET TOTAL PAID
    // =========================
    pub fn get_total_paid(env: Env, student: Address, tutor: Address) -> i128 {
        let key = DataKey::TotalPaid(student, tutor);

        env.storage()
            .instance()
            .get(&key)
            .unwrap_or(0)
    }

    // =========================
    // GET ALL PAYMENTS
    // =========================
    pub fn get_all_payments(env: Env) -> Vec<Payment> {
        env.storage()
            .instance()
            .get(&DataKey::PaymentHistory)
            .unwrap_or(Vec::new(&env))
    }
}