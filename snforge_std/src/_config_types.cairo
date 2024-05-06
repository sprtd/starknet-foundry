#[derive(Drop, Serde)]
struct AvailableGasConfig {
    gas: felt252
}

#[derive(Drop, Serde)]
enum BlockId {
    BlockTag: (),
    BlockHash: felt252,
    BlockNumber: felt252
}

#[derive(Drop, Serde)]
struct InlineForkConfig {
    url: ByteArray,
    block: BlockId
}

#[derive(Drop, Serde)]
enum ForkConfig {
    Inline: InlineForkConfig,
    Named: ByteArray
}

#[derive(Drop, Serde)]
struct FuzzerConfig {
    runs: Option<felt252>,
    seed: Option<felt252>
}

#[derive(Drop, Serde)]
enum String {
    Short: felt252,
    Normal: ByteArray
}

#[derive(Drop, Serde)]
struct ShouldPanicConfig {
    expected: Array<String>,
}

#[derive(Drop, Serde)]
struct IgnoreConfig {
    is_ignored: bool,
}
