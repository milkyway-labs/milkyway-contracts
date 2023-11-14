#[cfg(test)]
mod tests {
    use crate::state::BATCHES;
    use crate::tests::test_helper::init;

    use cosmwasm_std::Order;
    use milky_way::staking::BatchStatus;
    #[test]
    fn proper_instantiation() {
        let deps = init();

        let pending_batch = BATCHES
            .range(&deps.storage, None, None, Order::Descending)
            .find(|r| r.is_ok() && r.as_ref().unwrap().1.status == BatchStatus::Pending)
            .unwrap()
            .unwrap()
            .1;

        assert!(pending_batch.id == 1);
    }
}
