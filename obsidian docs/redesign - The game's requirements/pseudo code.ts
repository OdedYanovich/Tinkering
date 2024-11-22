const enum Expectations {
  Location,
  Combat,
  Options,
  Credit,
  Exit,
}
class VisualSettings{}
class PlayerHp{}
class EnemyHp{}
class CurrentRequiredSequence{}
class NextRequiredSequence{}
class Encounter{}
class WantedSequence{}
class WantedSequenceKind{}
class Token{}

class Management{
    e: Expectations|undefined;
    vs: VisualSettings|undefined;
    php: PlayerHp|undefined;
    ehp: EnemyHp|undefined;
    crs: CurrentRequiredSequence|undefined;
    nrs: NextRequiredSequence|undefined;
    enc: Encounter|undefined;
    ws: WantedSequence|undefined;
    wsk: WantedSequenceKind|undefined;
}
// Should take every worker and clerk as a parameter
const manager=(m: Management, backup: any)=>any

const stateClerkLocation=(ws: WantedSequence, backup: any)=>any
const stateClerkCombat=(ws: WantedSequence, php:PlayerHp,ehp:EnemyHp, crs:CurrentRequiredSequence,nrs:NextRequiredSequence, e:Encounter,backup: any)=>[PlayerHp,EnemyHp, CurrentRequiredSequence,NextRequiredSequence, any]
const stateClerkOptions=(ws: WantedSequence, vs:VisualSettings)=>[VisualSettings,any]
const stateClerkCredit=(ws: WantedSequence, backup: any)=>any
const stateClerkExit=(ws: WantedSequence, backup: any)=>any

const transitionClerkLocation=(vs:VisualSettings, hp:PlayerHp, backup: any)=>[WantedSequenceKind,any]
const transitionClerkCombat=(vs:VisualSettings, backup: any)=>[Encounter,EnemyHp,WantedSequenceKind]
const transitionClerkOptions=(vs:VisualSettings, backup: any)=>[WantedSequenceKind, any]
const transitionClerkCredit=(vs:VisualSettings, backup: any)=>[WantedSequenceKind, any]
const transitionClerkExit=(vs:VisualSettings, backup: any)=>[WantedSequenceKind, any]

const workerLocationIdentity=(vs:VisualSettings, backup: any)=>any
const workerCombatIdentity=(vs:VisualSettings, backup: any)=>any
const workerOptionsIdentity=(vs:VisualSettings, backup: any)=>any
const workerCreditIdentity=(vs:VisualSettings, backup: any)=>any
const workerExitIdentity=(vs:VisualSettings, backup: any)=>any
const workerDisplay=(backup: any)=>any

const workerWhiteList=(t:Token, ws:WantedSequence, wsk: WantedSequenceKind,backup: any)=>[WantedSequence,any]

console.log(Expectations.Combat)

