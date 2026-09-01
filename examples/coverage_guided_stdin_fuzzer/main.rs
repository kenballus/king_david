use {
    king_david::{
        Fuzzer,
        corpora::InMemoryCorpus,
        deciders::{AlwaysResult, AndSplitDecider, WasTerminatedBySignal},
        executors::AflppStdinExecutor,
        feedbacks::{AlwaysInterestingFeedback, AndSplitFeedback, EdgeCoverageFeedback},
        loggers::StdoutDebugLogger,
        mutators::ByteMutator,
    },
    rand::{SeedableRng, rngs::SmallRng},
    std::env::args_os,
};

fn main() {
    let mut args = args_os();
    let _ = args.next(); // skip argv[0]
    let args: Vec<_> = args.collect();
    assert!(args.len() > 0, "Missing arg. Tell me what to exec!");

    let mut rng = SmallRng::seed_from_u64(4 /* obtained by rolling a fair die */);

    let executor =
        AflppStdinExecutor::new(&args[0], args.as_slice(), "/tmp/.cur_input", None, None);
    let map_size = executor.map_size;

    let mut fuzzer = Fuzzer::new(
        AndSplitDecider::new(WasTerminatedBySignal::default(), AlwaysResult::default()),
        executor,
        AndSplitFeedback::new(
            AlwaysInterestingFeedback::default(),
            EdgeCoverageFeedback::new(map_size),
        ),
        ByteMutator::new(SmallRng::from_rng(&mut rng)),
        InMemoryCorpus::new(
            rng,
            vec![vec![]], /* one element in the corpus: the empty string */
        ),
        StdoutDebugLogger::default(),
    );
    fuzzer.go();
}
