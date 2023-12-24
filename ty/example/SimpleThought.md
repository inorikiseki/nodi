Based on (https://www.bilibili.com/video/BV1Pc411X74G/?spm_id_from=333.880.my_history.page.click&vd_source=0f9a1b0c757726736fe937c5bcf36b80)

// pesudo code.

Hero {
    isStun: bool
    isSilence: bool
    addBuff(buffType: BuffType) {
        buff = Buff.newBuff(bufftype); // Buff is just to trigger the isXXX to true.
    }
}

Is reasonable to written out isStun, isSilence these kind of boolean variables?
### the ultimate question is where it lies, why we create these variables?


After thinking a lot, the answer is bool is most of time unnessary most. 'cause boolean is just 
usually for stating exsitences. so instead of make such a variable, we track the real "existence".
if the state is easy to track out, we dont even need to create it.

1. Collections
enum BuffType{Stun, Silence, ...} // easy for extending.
Hero:
    buffs: [BuffType]
    hasBuff(self, _buff: BuffType): bool
        for buff in self.buffs:
            if buff == _buff:
                return true
        return false

    addBuff(self, buffType: BuffType) 
        if not self.buffs.contains(buff):
            buffs.append(buff)
    
    removeBuff(self, buffType: BuffType)
    ...


2. BitFlag (or using bitflag like structure.)

enum BuffType{Stun=1, Silence=1 << 1, ..}
Hero:
    buffFlag = 0
    hasBuff(self, buffType) -> bool
        return bool(self.buffFlag & buffType)
    addBuff(self, buffType)
        self.buffFlag |= buffType
    ...

3. Encapsualtion and abstraction
bit(x) => .. 1 << x ..
enum BuffType{Stun = bit(1), Silence = bit(2), ..}
enum PhysicalState{InAir = bit(1), OnFloor = bit(2), ...}
enum HeroState{canSprint = 1}
..
Hero:
    state: HeroState
    ..
StateManger ..
BuffManager: StateManager ..
PhysicalStateManager: StateManger ..
HeroState: StateManger
    stateFlag = 0
    bm: BuffManager
    psm: physicalStateManager
    is_in(state: HeroState)
        return update(state)
    update(state: HeroState)
        return .


bit(x) => 1 << x
enum BuffType{Stun=1, Silence ..}
enum PhysicalState{InAir=1, OnFloor, Lying .. }
enum ActionState{canAttack = 1, canSprint...}

StateManager:
    flag = 0
    updateRules = []
    setRule(state: int, fn)
        updateRule[type] = fn
    has(state) => updateRule[state](); return bool(flag & bit(state))
    add(state) => flag | bit(state)
    remove(state) => (flag & bit(state)) ^ bit(state)
    set(state, value) => if(value){add(state)}else{remove(state)}
Hero:
    state: HeroState
    ..
HeroState: StateManager
    hero: Hero..
    buffs = StateManger()
    physicalstates = StateManger()
    init():
        super.setRule(ActionState.canAttack, updateCanAttack)
        super.setRule(ActionState.canSprint, updateCanSprint)
    is_in(state: State) -> bool
        return super.has(state)
    updateCanAttack()
        super.set(canAttack, !buffs.has(BuffType.Silence) && hero.MP > 0)
    updateCanSprint()
        super.set(canSprint, !buffs.has(BuffType.Silence) && physicalstates.has(PhysicalState.OnFloor))
    
    
The more variable you create the more bound it is.
In a way, we must break the boundary between data and program, program itself
carry data.

it's worth calling meta.

now i am able to say, we can define, if there is not a restriction on orders.
we can posting refer and reducting to the front statement.

if there is nothing there.
we cant define.

if the order is bound, then nothing can came to it, causing we want 1 out of 0
acutally existence out of void.

if it bound to a preordering, we can't even start, cause nothing help or we can rely on
to define 0 and 1.
but we can rely on later profiling. POST referring is important.

so there is somekind of order, but there would be no order in defining the system.
cause 0 1 comes together. 0 cant live without 1.

meta
nodic : meta
0, 1
bit : [0 1]
true, false
bool'boolean : [true false]
u8 : [bit]^8
    !(physical based)
    (real implement...)
u16 : [u8]^2
u32 : [u8]^4, u32 : [u16]^2
f8 : [bit]^8
    (real implement)
f16 : [f8]^2
f32 : [f16]^2, f32 : [f8]^4
integer : u8 | u16 | u32
float : f8 | f16 | f32
number : integer | float
digit : [0 1 2 3 4 5 6 7 8 9]
letter : [
    a b c d e f g 
    h i j k l m n
    o p q   r s t
    u v w 
]  

all these symbols we used they must be disambiguated later. they need a cyclalic
dependence, as 1 depends on 0, 0 depends on 1.
when the context is weak, we may want to introducing some disamibguation manually,
but when the context and corpus is enough, the compiler has enough comprehension,
we dont even need to specify on our own, most of the cases, it can figure out as 
human do. we should explain our intention as needed as we do it for our fellows.

