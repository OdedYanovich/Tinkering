const enum Expectations {
  Location,
  Combat,
  Options,
  Credit,
  Exit,
}
class VisualSettings { }
class PlayerHp { }
class EnemyHp { }
class CurrentRequiredSequence { }
class NextRequiredSequence { }
class Encounter { }
class Sequence { }
class WantedSequenceKind {
  user: sequenceUser
}
class sequenceUser { }
class Token { }
class WantedTokens { }

class Management {
  e: Expectations;
  vs: VisualSettings;
  php: PlayerHp;
  ehp: EnemyHp;
  crs: CurrentRequiredSequence;
  nrs: NextRequiredSequence;
  enc: Encounter;
  ws: Sequence;
  wsk: WantedSequenceKind;
}
// Should take every worker and clerk as a parameter
const manager = (m: Management, other: any) => other

const stateClerkLocation = (s: Sequence, other: any) => other
const stateClerkCombat = (s: Sequence, php: PlayerHp, ehp: EnemyHp, crs: CurrentRequiredSequence, nrs: NextRequiredSequence, e: Encounter, other: any) => [PlayerHp, EnemyHp, CurrentRequiredSequence, NextRequiredSequence, other]
const stateClerkOptions = (s: Sequence, vs: VisualSettings, other: any) => [VisualSettings, other]
const stateClerkCredit = (s: Sequence, other: any) => other
const stateClerkExit = (s: Sequence, other: any) => other

const transitionClerkLocation = (vs: VisualSettings, hp: PlayerHp, other: any) => [WantedSequenceKind, other]
const transitionClerkCombat = (vs: VisualSettings, backup: any) => [Encounter, EnemyHp, WantedSequenceKind]
const transitionClerkOptions = (vs: VisualSettings, other: any) => [WantedSequenceKind, other]
const transitionClerkCredit = (vs: VisualSettings, other: any) => [WantedSequenceKind, other]
const transitionClerkExit = (vs: VisualSettings, other: any) => [WantedSequenceKind, other]

const blueCollarLocationIdentity = (vs: VisualSettings, other: any) => other
const blueCollarCombatIdentity = (vs: VisualSettings, other: any) => other
const blueCollarOptionsIdentity = (vs: VisualSettings, other: any) => other
const blueCollarCreditIdentity = (vs: VisualSettings, other: any) => other
const blueCollarExitIdentity = (vs: VisualSettings, other: any) => other
const blueCollarDisplay = (other: any) => other

const blueCollarWhiteList = (t: Token, wt: WantedTokens, other: any) => [Token, other]
const blueCollarSequencer = (t: Token, ws: Sequence, wsk: WantedSequenceKind, other: any) => [Sequence, other]

console.log(Expectations.Combat)

