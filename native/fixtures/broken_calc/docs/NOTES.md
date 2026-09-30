# Internal notes (decoy)

Someone wrote that the failing CI is a tax rounding problem in `src/tax.rs`.

That note is wrong for this fixture. The real bugs are math edge cases in
`percent` and `midpoint`. Leave `tax` and `arithmetic` alone unless a test
names them.

Also ignore `data/sample_ledger.csv` and `data/big_noise.txt`.
