type Sequence
type SequenceUser
type WantedSequenceKind {
  User(SequenceUser)
}
type CurrentRequiredSequence 
type NextRequiredSequence 
type VisualSettings
type PlayerHp {
  Increase
  Decrease
}
type EnemyHp {
  DecreaseBy(Int)
}
type Encounter
type Token
type WantedTokens

type LocationClerk = fn(Sequence) -> Nil
type CombatClerk = fn(Sequence, PlayerHp, EnemyHp, CurrentRequiredSequence, NextRequiredSequence, Encounter) -> #(PlayerHp, EnemyHp, CurrentRequiredSequence, NextRequiredSequence)
type OptionClerk = fn(Sequence, VisualSettings) -> VisualSettings
type CreditClerk = fn(Sequence) -> Nil
type ExitClerk = fn(Sequence) -> Nil

type LocationOrganizer = fn(VisualSettings, PlayerHp) -> WantedSequenceKind
type CombatOrganizer = fn(VisualSettings) -> #(Encounter, EnemyHp, WantedSequenceKind)
type OptionOrganizer = fn(VisualSettings, ) -> WantedSequenceKind
type CreditOrganizer = fn(VisualSettings, ) -> WantedSequenceKind
type ExitOrganizer = fn(VisualSettings, ) -> WantedSequenceKind

type BlueCollarLocationIdentity = fn(VisualSettings) -> Nil
type BlueCollarCombatIdentity = fn(VisualSettings) -> Nil
type BlueCollarOptionsIdentity = fn(VisualSettings) -> Nil
type BlueCollarCreditIdentity = fn(VisualSettings) -> Nil
type BlueCollarExitIdentity = fn(VisualSettings) -> Nil
type BlueCollarDisplay = fn() -> Nil

type BlueCollarWhiteList = fn(Token, WantedTokens) -> Nil
type BlueCollarSequencer = fn(Token, Sequence, WantedSequenceKind) -> Nil

// type Management {
//   e: Expectations
//   vs: VisualSettings
//   php: PlayerHp
//   ehp: EnemyHp
//   crs: CurrentRequiredSequence
//   nrs: NextRequiredSequence
//   enc: Encounter
//   ws: Sequence
//   wsk: WantedSequenceKind
// }
// // Should take every worker and clerk as a parameter
// const manager = (m: Management, other: any) => other
