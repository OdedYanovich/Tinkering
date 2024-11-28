type States {
  Location
  Combat
  Options
  Exit
}
type Sequence
type SequenceUser
type WantedSequenceKind = #(SequenceUser, Int)
type CurrentRequiredSequence 
type NextRequiredSequence 
type VisualSettings
type PlayerHp = Int
type EnemyHp = Int
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
type EnemyHpChange = fn(Int, Int) -> Int

pub type Management = #(
  States,
  VisualSettings,
  PlayerHp,
  EnemyHp,
  CurrentRequiredSequence,
  NextRequiredSequence,
  Encounter,
  Sequence,
  WantedSequenceKind,
  LocationClerk,
  CombatClerk,
  OptionClerk,
  CreditClerk,
  ExitClerk,
  LocationOrganizer,
  CombatOrganizer,
  OptionOrganizer,
  CreditOrganizer,
  ExitOrganizer,
  BlueCollarLocationIdentity,
  BlueCollarCombatIdentity,
  BlueCollarOptionsIdentity,
  BlueCollarCreditIdentity,
  BlueCollarExitIdentity,
  BlueCollarDisplay,
  BlueCollarWhiteList,
  BlueCollarSequencer,
  EnemyHpChange,
)
pub type Manager = fn(Management, Token) -> Management
