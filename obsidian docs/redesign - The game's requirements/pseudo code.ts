const enum Expectations {
  Location,
  Combat,
  Options,
  Credit,
  Exit,
}
interface VisualSettings { }
interface PlayerHp {
  decrease
  increase
 }
interface EnemyHp { 
  decrease(amountMultiplier)
}
interface CurrentRequiredSequence { }
interface NextRequiredSequence { }
interface Encounter { }
interface Sequence { }
interface WantedSequenceKind {
  user: sequenceUser
}
interface sequenceUser { }
interface Token { }
interface WantedTokens { }

const ClerkLocation = (s: Sequence, other: any) => other
const ClerkCombat = (s: Sequence, php: PlayerHp, ehp: EnemyHp, crs: CurrentRequiredSequence, nrs: NextRequiredSequence, e: Encounter, other: any) => [PlayerHp, EnemyHp, CurrentRequiredSequence, NextRequiredSequence, other]
const ClerkOptions = (s: Sequence, vs: VisualSettings, other: any) => [VisualSettings, other]
const ClerkCredit = (s: Sequence, other: any) => other
const ClerkExit = (s: Sequence, other: any) => other

const OrganizerLocation = (vs: VisualSettings, hp: PlayerHp, other: any) => [WantedSequenceKind, other]
const OrganizerCombat = (vs: VisualSettings, backup: any) => [Encounter, EnemyHp, WantedSequenceKind]
const OrganizerOptions = (vs: VisualSettings, other: any) => [WantedSequenceKind, other]
const OrganizerCredit = (vs: VisualSettings, other: any) => [WantedSequenceKind, other]
const OrganizerExit = (vs: VisualSettings, other: any) => [WantedSequenceKind, other]

const blueCollarLocationIdentity = (vs: VisualSettings, other: any) => other
const blueCollarCombatIdentity = (vs: VisualSettings, other: any) => other
const blueCollarOptionsIdentity = (vs: VisualSettings, other: any) => other
const blueCollarCreditIdentity = (vs: VisualSettings, other: any) => other
const blueCollarExitIdentity = (vs: VisualSettings, other: any) => other
const blueCollarDisplay = (other: any) => other

const blueCollarWhiteList = (t: Token, wt: WantedTokens, other: any) => [Token, other]
const blueCollarSequencer = (t: Token, ws: Sequence, wsk: WantedSequenceKind, other: any) => [Sequence, other]

console.log(Expectations.Combat)


// class Management {
//   e: Expectations;
//   vs: VisualSettings;
//   php: PlayerHp;
//   ehp: EnemyHp;
//   crs: CurrentRequiredSequence;
//   nrs: NextRequiredSequence;
//   enc: Encounter;
//   ws: Sequence;
//   wsk: WantedSequenceKind;
// }
// // Should take every worker and clerk as a parameter
// const manager = (m: Management, other: any) => other
