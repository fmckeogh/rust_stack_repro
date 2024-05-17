#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use AArch64_S1EPD::*;
use AArch64_GetS1TLBContext::*;
use AArch64_TTWalkMECID::*;
use AArch64_S1DCacheEnabled::*;
use AArch64_S1NextWalkStateLeaf::*;
use AArch64_GetVARange::*;
use S1TLBCache::*;
use AArch64_OAOutOfRange::*;
use AArch64_S2Translate::*;
use u__UNKNOWN_bits::*;
use NormalNCMemAttr::*;
use AArch64_S1SLTTEntryAddress::*;
use EL2Enabled::*;
use CreateAccDescTTEUpdate::*;
use TranslationSize::*;
use AArch64_ContiguousBitFaults::*;
use u__UNKNOWN_TTWState::*;
use FetchDescriptor::*;
use StageOA::*;
use AArch64_MemSwapTableDesc::*;
use u_get_HCR_EL2_Type_VM::*;
use AArch64_BlocknTFaults::*;
use S1TLBLookup::*;
use HasUnprivileged::*;
use AArch64_S1InitialTTWState::*;
use CreateAccDescS1TTW::*;
use EffectiveShareability::*;
use AArch64_TTEntryAddress::*;
use AArch64_S1AMECFault::*;
use Unreachable::*;
use ContiguousSize::*;
use u__id::*;
use AArch64_S1NextWalkStateTable::*;
use u__UNKNOWN_AddressDescriptor::*;
use AArch64_DecodeDescriptorType::*;
use common::*;
pub fn AArch64_S1Walk<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    walkparams: ProductTypeef284266e139aee2,
    va: u64,
    regime: u32,
    accdesc: ProductType9878976b5bcce9c9,
    N: i128,
) -> ProductType4b99944cd5e0b59d {
    #[derive(Default)]
    struct FunctionState {
        ga_15127: ProductTypeda0231e9dc169f81,
        gs_19727: bool,
        tlbcontext: ProductTypec0d0fb0603850c4c,
        gs_19644: bool,
        gs_19689: bool,
        gs_19643: bool,
        ga_14977: ProductTypef170cab34335b70c,
        gs_19641: bool,
        gs_19761: bool,
        gs_19645: bool,
        descaccess: ProductType9878976b5bcce9c9,
        skl: u8,
        new_descriptor: Bits,
        gs_19636: bool,
        translation_info: ProductTypeb525737120e184b3,
        gs_19706: bool,
        ga_14983: ProductTypef170cab34335b70c,
        gs_19730: bool,
        walkaddress: ProductTypece7c66ccb2cab13e,
        ga_15080: ProductTypedc31059ca7e2391c,
        ga_15024: ProductTypeb4cea7287e2eb9d6,
        tlbrecord: ProductTypee47dd77b186df56e,
        descaddress: ProductTypeda0231e9dc169f81,
        gs_19687: bool,
        gs_19660: bool,
        gs_19675: bool,
        ga_15153: ProductTypef170cab34335b70c,
        gs_19669: bool,
        gs_19705: bool,
        gs_19640: bool,
        desctype: u32,
        ga_15015: ProductTypeb4cea7287e2eb9d6,
        gs_19677: bool,
        gs_19760: bool,
        Nshadow_322: i128,
        descriptor: Bits,
        walkaccess: ProductType9878976b5bcce9c9,
        gs_19658: bool,
        startlevel: i128,
        fault: ProductType1d757adad216cdef,
        s2fault: ProductType1d757adad216cdef,
        gs_19659: bool,
        gs_19795: bool,
        oa: ProductTypeda0231e9dc169f81,
        gs_19802: bool,
        return_value: ProductType4b99944cd5e0b59d,
        s2fs1mro: bool,
        s2walkaddress: ProductTypece7c66ccb2cab13e,
        gs_19764: bool,
        ga_14948: ProductType96e7acababe246a1,
        gs_19798: bool,
        gs_19633: bool,
        u_903: ProductTypeda0231e9dc169f81,
        gs_19794: bool,
        ga_15093: ProductTypeb4cea7287e2eb9d6,
        descpaddr: ProductTypece7c66ccb2cab13e,
        walkstate: ProductType96e7acababe246a1,
        ga_14999: ProductTypedc31059ca7e2391c,
        gs_19637: bool,
        gs_19651: bool,
        gs_19803: bool,
        gs_19642: bool,
        tlbentry: ProductTypeeb828c17bbe5e68,
        gs_19624: bool,
        fault_in: ProductType1d757adad216cdef,
        walkparams: ProductTypeef284266e139aee2,
        va: u64,
        regime: u32,
        accdesc: ProductType9878976b5bcce9c9,
        N: i128,
    }
    let fn_state = FunctionState {
        fault_in,
        walkparams,
        va,
        regime,
        accdesc,
        N,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_0_0: read-var N:i
        let s_0_0: i128 = fn_state.N;
        // D s_0_1: write-var Nshadow#322 <= s_0_0
        fn_state.Nshadow_322 = s_0_0;
        // D s_0_2: read-var fault_in:struct
        let s_0_2: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_3: write-var fault <= s_0_2
        fn_state.fault = s_0_2;
        // C s_0_4: const #19088u : u32
        let s_0_4: u32 = 19088;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: bool = {
            let value = state.read_register::<bool>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // N s_0_6: branch s_0_5 b137 b1
        if s_0_5 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_2_0: read-var regime:u32
        let s_2_0: u32 = fn_state.regime;
        // D s_2_1: call HasUnprivileged(s_2_0)
        let s_2_1: bool = HasUnprivileged(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b136 b3
        if s_2_1 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#19624 <= s_3_0
        fn_state.gs_19624 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_4_0: read-var gs#19624:u8
        let s_4_0: bool = fn_state.gs_19624;
        // N s_4_1: branch s_4_0 b135 b5
        if s_4_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_5_0: read-var accdesc.25:struct
        let s_5_0: u32 = fn_state.accdesc._25;
        // D s_5_1: read-var walkparams:struct
        let s_5_1: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_5_2: read-var va:u64
        let s_5_2: u64 = fn_state.va;
        // D s_5_3: read-var regime:u32
        let s_5_3: u32 = fn_state.regime;
        // D s_5_4: call AArch64_S1InitialTTWState(s_5_1, s_5_2, s_5_3, s_5_0)
        let s_5_4: ProductType96e7acababe246a1 = AArch64_S1InitialTTWState(
            state,
            tracer,
            s_5_1,
            s_5_2,
            s_5_3,
            s_5_0,
        );
        // D s_5_5: write-var walkstate <= s_5_4
        fn_state.walkstate = s_5_4;
        // D s_5_6: read-var walkstate.6:struct
        let s_5_6: i128 = fn_state.walkstate._6;
        // D s_5_7: write-var startlevel <= s_5_6
        fn_state.startlevel = s_5_6;
        // C s_5_8: const #0u : u8
        let s_5_8: u8 = 0;
        // D s_5_9: write-var skl <= s_5_8
        fn_state.skl = s_5_8;
        // D s_5_10: read-var va:u64
        let s_5_10: u64 = fn_state.va;
        // D s_5_11: write-var walkaddress.7 <= s_5_10
        fn_state.walkaddress._7 = s_5_10;
        // D s_5_12: read-var walkparams.10:struct
        let s_5_12: bool = fn_state.walkparams._10;
        // D s_5_13: read-var accdesc.25:struct
        let s_5_13: u32 = fn_state.accdesc._25;
        // D s_5_14: read-var regime:u32
        let s_5_14: u32 = fn_state.regime;
        // D s_5_15: call AArch64_TTWalkMECID(s_5_12, s_5_14, s_5_13)
        let s_5_15: u16 = AArch64_TTWalkMECID(state, tracer, s_5_12, s_5_14, s_5_13);
        // D s_5_16: write-var walkaddress.1 <= s_5_15
        fn_state.walkaddress._1 = s_5_15;
        // D s_5_17: read-var regime:u32
        let s_5_17: u32 = fn_state.regime;
        // D s_5_18: call AArch64_S1DCacheEnabled(s_5_17)
        let s_5_18: bool = AArch64_S1DCacheEnabled(state, tracer, s_5_17);
        // D s_5_19: not s_5_18
        let s_5_19: bool = !s_5_18;
        // N s_5_20: branch s_5_19 b134 b6
        if s_5_19 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_6_0: read-var walkstate.7:struct
        let s_6_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_6_1: write-var walkaddress.2 <= s_6_0
        fn_state.walkaddress._2 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_7_0: read-var regime:u32
        let s_7_0: u32 = fn_state.regime;
        // C s_7_1: const #4u : u32
        let s_7_1: u32 = 4;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b133 b8
        if s_7_2 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#19658 <= s_8_0
        fn_state.gs_19658 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_9_0: read-var gs#19658:u8
        let s_9_0: bool = fn_state.gs_19658;
        // N s_9_1: branch s_9_0 b132 b10
        if s_9_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#19659 <= s_10_0
        fn_state.gs_19659 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_11_0: read-var gs#19659:u8
        let s_11_0: bool = fn_state.gs_19659;
        // N s_11_1: branch s_11_0 b131 b12
        if s_11_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#19660 <= s_12_0
        fn_state.gs_19660 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_13_0: read-var gs#19660:u8
        let s_13_0: bool = fn_state.gs_19660;
        // N s_13_1: branch s_13_0 b130 b14
        if s_13_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_14_0: read-var walkaddress.2:struct
        let s_14_0: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // D s_14_1: call EffectiveShareability(s_14_0)
        let s_14_1: u32 = EffectiveShareability(state, tracer, s_14_0);
        // D s_14_2: write-var walkaddress.2.5 <= s_14_1
        fn_state.walkaddress._2._5 = s_14_1;
        // N s_14_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_15_0: read-var tlbcontext:struct
        let s_15_0: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_15_1: write-var walkaddress.6 <= s_15_0
        fn_state.walkaddress._6 = s_15_0;
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // D s_15_3: write-var s2fs1mro <= s_15_2
        fn_state.s2fs1mro = s_15_2;
        // D s_15_4: read-var walkstate.6:struct
        let s_15_4: i128 = fn_state.walkstate._6;
        // D s_15_5: read-var walkstate.0:struct
        let s_15_5: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_15_6: read-var walkparams:struct
        let s_15_6: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_15_7: read-var va:u64
        let s_15_7: u64 = fn_state.va;
        // D s_15_8: call AArch64_S1SLTTEntryAddress(s_15_4, s_15_6, s_15_7, s_15_5)
        let s_15_8: ProductTypeda0231e9dc169f81 = AArch64_S1SLTTEntryAddress(
            state,
            tracer,
            s_15_4,
            s_15_6,
            s_15_7,
            s_15_5,
        );
        // D s_15_9: write-var descaddress <= s_15_8
        fn_state.descaddress = s_15_8;
        // D s_15_10: read-var descaddress.0:struct
        let s_15_10: u64 = fn_state.descaddress._0;
        // D s_15_11: read-var walkparams.3:struct
        let s_15_11: bool = fn_state.walkparams._3;
        // D s_15_12: read-var walkparams.28:struct
        let s_15_12: u8 = fn_state.walkparams._28;
        // D s_15_13: read-var walkparams.36:struct
        let s_15_13: u32 = fn_state.walkparams._36;
        // D s_15_14: call AArch64_OAOutOfRange(s_15_10, s_15_11, s_15_12, s_15_13)
        let s_15_14: bool = AArch64_OAOutOfRange(
            state,
            tracer,
            s_15_10,
            s_15_11,
            s_15_12,
            s_15_13,
        );
        // N s_15_15: branch s_15_14 b129 b16
        if s_15_14 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_17_0: read-var walkstate.6:struct
        let s_17_0: i128 = fn_state.walkstate._6;
        // D s_17_1: write-var fault.9 <= s_17_0
        fn_state.fault._9 = s_17_0;
        // D s_17_2: read-var descaddress:struct
        let s_17_2: ProductTypeda0231e9dc169f81 = fn_state.descaddress;
        // D s_17_3: write-var walkaddress.3 <= s_17_2
        fn_state.walkaddress._3 = s_17_2;
        // D s_17_4: read-var walkstate.6:struct
        let s_17_4: i128 = fn_state.walkstate._6;
        // D s_17_5: read-var startlevel:i
        let s_17_5: i128 = fn_state.startlevel;
        // D s_17_6: cmp-eq s_17_4 s_17_5
        let s_17_6: bool = ((s_17_4) == (s_17_5));
        // D s_17_7: read-var va:u64
        let s_17_7: u64 = fn_state.va;
        // D s_17_8: call AArch64_GetVARange(s_17_7)
        let s_17_8: u32 = AArch64_GetVARange(state, tracer, s_17_7);
        // D s_17_9: read-var accdesc:struct
        let s_17_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_17_10: call CreateAccDescS1TTW(s_17_6, s_17_8, s_17_9)
        let s_17_10: ProductType9878976b5bcce9c9 = CreateAccDescS1TTW(
            state,
            tracer,
            s_17_6,
            s_17_8,
            s_17_9,
        );
        // D s_17_11: write-var walkaccess <= s_17_10
        fn_state.walkaccess = s_17_10;
        // C s_17_12: const #() : ()
        let s_17_12: () = ();
        // D s_17_13: create-sum enum = 0:"s_17_12"
        let s_17_13: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_17_12);
        // C s_17_14: const #() : ()
        let s_17_14: () = ();
        // D s_17_15: create-sum enum = 0:"s_17_14"
        let s_17_15: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_17_14);
        // D s_17_16: read-var walkaddress.7:struct
        let s_17_16: u64 = fn_state.walkaddress._7;
        // D s_17_17: read-var walkstate.6:struct
        let s_17_17: i128 = fn_state.walkstate._6;
        // D s_17_18: create-sum enum = 1:"s_17_17"
        let s_17_18: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_1(s_17_17);
        // C s_17_19: const #() : ()
        let s_17_19: () = ();
        // D s_17_20: create-sum enum = 0:"s_17_19"
        let s_17_20: SumType3cca557f9e907281 = SumType3cca557f9e907281::_0(s_17_19);
        // D s_17_21: read-var walkparams:struct
        let s_17_21: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_17_22: create-sum enum = 1:"s_17_21"
        let s_17_22: SumTypefc0aa8a49e605a17 = SumTypefc0aa8a49e605a17::_1(s_17_21);
        // C s_17_23: const #() : ()
        let s_17_23: () = ();
        // D s_17_24: create-sum enum = 0:"s_17_23"
        let s_17_24: SumType3436044442b382d9 = SumType3436044442b382d9::_0(s_17_23);
        // D s_17_25: read-var walkaddress.2:struct
        let s_17_25: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // D s_17_26: read-var regime:u32
        let s_17_26: u32 = fn_state.regime;
        // D s_17_27: create-product struct = ["s_17_15", "s_17_25", "s_17_26", "s_17_18", "s_17_22", "s_17_20", "s_17_24", "s_17_16", "s_17_13"]
        let s_17_27: ProductTypeb525737120e184b3 = ProductTypeb525737120e184b3 {
            _0: s_17_15,
            _1: s_17_25,
            _2: s_17_26,
            _3: s_17_18,
            _4: s_17_22,
            _5: s_17_20,
            _6: s_17_24,
            _7: s_17_16,
            _8: s_17_13,
        };
        // D s_17_28: write-var translation_info <= s_17_27
        fn_state.translation_info = s_17_27;
        // D s_17_29: read-var regime:u32
        let s_17_29: u32 = fn_state.regime;
        // C s_17_30: const #4u : u32
        let s_17_30: u32 = 4;
        // D s_17_31: cmp-eq s_17_29 s_17_30
        let s_17_31: bool = ((s_17_29) == (s_17_30));
        // N s_17_32: branch s_17_31 b128 b18
        if s_17_31 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#19669 <= s_18_0
        fn_state.gs_19669 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_19_0: read-var gs#19669:u8
        let s_19_0: bool = fn_state.gs_19669;
        // N s_19_1: branch s_19_0 b119 b20
        if s_19_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_20_0: read-var Nshadow#322:i
        let s_20_0: i128 = fn_state.Nshadow_322;
        // D s_20_1: call __id(s_20_0)
        let s_20_1: i128 = u__id(state, tracer, s_20_0);
        // C s_20_2: const #32s : i
        let s_20_2: i128 = 32;
        // D s_20_3: cmp-eq s_20_1 s_20_2
        let s_20_3: bool = ((s_20_1) == (s_20_2));
        // N s_20_4: branch s_20_3 b118 b21
        if s_20_3 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_21_0: read-var Nshadow#322:i
        let s_21_0: i128 = fn_state.Nshadow_322;
        // D s_21_1: call __id(s_21_0)
        let s_21_1: i128 = u__id(state, tracer, s_21_0);
        // C s_21_2: const #64s : i
        let s_21_2: i128 = 64;
        // D s_21_3: cmp-eq s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) == (s_21_2));
        // D s_21_4: write-var gs#19675 <= s_21_3
        fn_state.gs_19675 = s_21_3;
        // N s_21_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_22_0: read-var gs#19675:u8
        let s_22_0: bool = fn_state.gs_19675;
        // N s_22_1: branch s_22_0 b117 b23
        if s_22_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_23_0: read-var Nshadow#322:i
        let s_23_0: i128 = fn_state.Nshadow_322;
        // D s_23_1: call __id(s_23_0)
        let s_23_1: i128 = u__id(state, tracer, s_23_0);
        // C s_23_2: const #128s : i
        let s_23_2: i128 = 128;
        // D s_23_3: cmp-eq s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) == (s_23_2));
        // D s_23_4: write-var gs#19677 <= s_23_3
        fn_state.gs_19677 = s_23_3;
        // N s_23_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_24_0: read-var gs#19677:u8
        let s_24_0: bool = fn_state.gs_19677;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // D s_24_2: read-var walkparams.9:struct
        let s_24_2: bool = fn_state.walkparams._9;
        // D s_24_3: read-var Nshadow#322:i
        let s_24_3: i128 = fn_state.Nshadow_322;
        // D s_24_4: cast reint s_24_3 -> i64
        let s_24_4: i64 = (s_24_3 as i64);
        // D s_24_5: read-var walkaddress:struct
        let s_24_5: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_24_6: read-var walkaccess:struct
        let s_24_6: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_24_7: read-var fault:struct
        let s_24_7: ProductType1d757adad216cdef = fn_state.fault;
        // D s_24_8: read-var translation_info:struct
        let s_24_8: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_24_9: call FetchDescriptor(s_24_2, s_24_5, s_24_6, s_24_7, s_24_4, s_24_8)
        let s_24_9: ProductTypeb4cea7287e2eb9d6 = FetchDescriptor(
            state,
            tracer,
            s_24_2,
            s_24_5,
            s_24_6,
            s_24_7,
            s_24_4,
            s_24_8,
        );
        // D s_24_10: write-var ga#15024 <= s_24_9
        fn_state.ga_15024 = s_24_9;
        // D s_24_11: read-var ga#15024.0:struct
        let s_24_11: ProductType1d757adad216cdef = fn_state.ga_15024._0;
        // D s_24_12: read-var ga#15024.1:struct
        let s_24_12: Bits = fn_state.ga_15024._1;
        // D s_24_13: write-var fault <= s_24_11
        fn_state.fault = s_24_11;
        // D s_24_14: write-var descriptor <= s_24_12
        fn_state.descriptor = s_24_12;
        // N s_24_15: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_25_0: read-var fault.16:struct
        let s_25_0: u32 = fn_state.fault._16;
        // C s_25_1: const #0u : u32
        let s_25_1: u32 = 0;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // N s_25_3: branch s_25_2 b116 b26
        if s_25_2 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_27_0: read-var descriptor:bv
        let s_27_0: Bits = fn_state.descriptor;
        // D s_27_1: write-var new_descriptor <= s_27_0
        fn_state.new_descriptor = s_27_0;
        // D s_27_2: read-var walkparams.3:struct
        let s_27_2: bool = fn_state.walkparams._3;
        // D s_27_3: read-var walkparams.7:struct
        let s_27_3: bool = fn_state.walkparams._7;
        // D s_27_4: read-var walkparams.36:struct
        let s_27_4: u32 = fn_state.walkparams._36;
        // D s_27_5: read-var walkstate.6:struct
        let s_27_5: i128 = fn_state.walkstate._6;
        // D s_27_6: read-var descriptor:bv
        let s_27_6: Bits = fn_state.descriptor;
        // D s_27_7: call AArch64_DecodeDescriptorType(s_27_6, s_27_2, s_27_3, s_27_4, s_27_5)
        let s_27_7: u32 = AArch64_DecodeDescriptorType(
            state,
            tracer,
            s_27_6,
            s_27_2,
            s_27_3,
            s_27_4,
            s_27_5,
        );
        // D s_27_8: write-var desctype <= s_27_7
        fn_state.desctype = s_27_7;
        // C s_27_9: const #0u : u32
        let s_27_9: u32 = 0;
        // D s_27_10: read-var desctype:u32
        let s_27_10: u32 = fn_state.desctype;
        // D s_27_11: cmp-eq s_27_9 s_27_10
        let s_27_11: bool = ((s_27_9) == (s_27_10));
        // D s_27_12: not s_27_11
        let s_27_12: bool = !s_27_11;
        // N s_27_13: branch s_27_12 b111 b28
        if s_27_12 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_28_0: read-var walkstate:struct
        let s_28_0: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_28_1: read-var s2fs1mro:u8
        let s_28_1: bool = fn_state.s2fs1mro;
        // D s_28_2: read-var regime:u32
        let s_28_2: u32 = fn_state.regime;
        // D s_28_3: read-var walkparams:struct
        let s_28_3: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_28_4: read-var descriptor:bv
        let s_28_4: Bits = fn_state.descriptor;
        // D s_28_5: call AArch64_S1NextWalkStateTable(s_28_0, s_28_1, s_28_2, s_28_3, s_28_4)
        let s_28_5: ProductType96e7acababe246a1 = AArch64_S1NextWalkStateTable(
            state,
            tracer,
            s_28_0,
            s_28_1,
            s_28_2,
            s_28_3,
            s_28_4,
        );
        // D s_28_6: write-var walkstate <= s_28_5
        fn_state.walkstate = s_28_5;
        // D s_28_7: read-var walkparams.3:struct
        let s_28_7: bool = fn_state.walkparams._3;
        // D s_28_8: cast zx s_28_7 -> bv
        let s_28_8: Bits = Bits::new(s_28_7 as u128, 1u16);
        // C s_28_9: const #1u : u8
        let s_28_9: bool = true;
        // C s_28_10: cast zx s_28_9 -> bv
        let s_28_10: Bits = Bits::new(s_28_9 as u128, 1u16);
        // D s_28_11: cmp-eq s_28_8 s_28_10
        let s_28_11: bool = ((s_28_8) == (s_28_10));
        // N s_28_12: branch s_28_11 b110 b29
        if s_28_11 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_29_0: const #0u : u8
        let s_29_0: u8 = 0;
        // D s_29_1: write-var skl <= s_29_0
        fn_state.skl = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_30_0: read-var walkstate.6:struct
        let s_30_0: i128 = fn_state.walkstate._6;
        // D s_30_1: read-var walkparams.3:struct
        let s_30_1: bool = fn_state.walkparams._3;
        // D s_30_2: read-var walkparams.36:struct
        let s_30_2: u32 = fn_state.walkparams._36;
        // D s_30_3: read-var walkparams.37:struct
        let s_30_3: u8 = fn_state.walkparams._37;
        // D s_30_4: read-var walkstate.0:struct
        let s_30_4: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_30_5: read-var skl:u8
        let s_30_5: u8 = fn_state.skl;
        // D s_30_6: read-var va:u64
        let s_30_6: u64 = fn_state.va;
        // D s_30_7: call AArch64_TTEntryAddress(s_30_0, s_30_1, s_30_5, s_30_2, s_30_3, s_30_6, s_30_4)
        let s_30_7: ProductTypeda0231e9dc169f81 = AArch64_TTEntryAddress(
            state,
            tracer,
            s_30_0,
            s_30_1,
            s_30_5,
            s_30_2,
            s_30_3,
            s_30_6,
            s_30_4,
        );
        // D s_30_8: write-var descaddress <= s_30_7
        fn_state.descaddress = s_30_7;
        // D s_30_9: read-var descaddress.0:struct
        let s_30_9: u64 = fn_state.descaddress._0;
        // D s_30_10: read-var walkparams.3:struct
        let s_30_10: bool = fn_state.walkparams._3;
        // D s_30_11: read-var walkparams.28:struct
        let s_30_11: u8 = fn_state.walkparams._28;
        // D s_30_12: read-var walkparams.36:struct
        let s_30_12: u32 = fn_state.walkparams._36;
        // D s_30_13: call AArch64_OAOutOfRange(s_30_9, s_30_10, s_30_11, s_30_12)
        let s_30_13: bool = AArch64_OAOutOfRange(
            state,
            tracer,
            s_30_9,
            s_30_10,
            s_30_11,
            s_30_12,
        );
        // N s_30_14: branch s_30_13 b109 b31
        if s_30_13 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_31_0: read-var walkparams.13:struct
        let s_31_0: bool = fn_state.walkparams._13;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // N s_31_5: branch s_31_4 b108 b32
        if s_31_4 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_32_0: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_33_0: read-var walkparams.3:struct
        let s_33_0: bool = fn_state.walkparams._3;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // N s_33_5: branch s_33_4 b107 b34
        if s_33_4 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#19705 <= s_34_0
        fn_state.gs_19705 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_35_0: read-var gs#19705:u8
        let s_35_0: bool = fn_state.gs_19705;
        // N s_35_1: branch s_35_0 b106 b36
        if s_35_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#19706 <= s_36_0
        fn_state.gs_19706 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_37_0: read-var gs#19706:u8
        let s_37_0: bool = fn_state.gs_19706;
        // N s_37_1: branch s_37_0 b105 b38
        if s_37_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_39_0: read-var new_descriptor:bv
        let s_39_0: Bits = fn_state.new_descriptor;
        // D s_39_1: read-var descriptor:bv
        let s_39_1: Bits = fn_state.descriptor;
        // D s_39_2: cmp-ne s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) != (s_39_1));
        // N s_39_3: branch s_39_2 b91 b40
        if s_39_2 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_40_0: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_41_0: read-var new_descriptor:bv
        let s_41_0: Bits = fn_state.new_descriptor;
        // D s_41_1: read-var descriptor:bv
        let s_41_1: Bits = fn_state.descriptor;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // N s_41_3: branch s_41_2 b42 b27
        if s_41_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_42_0: read-var desctype:u32
        let s_42_0: u32 = fn_state.desctype;
        // C s_42_1: const #1u : u32
        let s_42_1: u32 = 1;
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // N s_42_3: branch s_42_2 b43 b17
        if s_42_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_43_0: read-var walkparams.3:struct
        let s_43_0: bool = fn_state.walkparams._3;
        // D s_43_1: read-var walkparams.36:struct
        let s_43_1: u32 = fn_state.walkparams._36;
        // D s_43_2: read-var va:u64
        let s_43_2: u64 = fn_state.va;
        // D s_43_3: read-var walkstate:struct
        let s_43_3: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_43_4: call StageOA(s_43_2, s_43_0, s_43_1, s_43_3)
        let s_43_4: ProductTypeda0231e9dc169f81 = StageOA(
            state,
            tracer,
            s_43_2,
            s_43_0,
            s_43_1,
            s_43_3,
        );
        // D s_43_5: write-var u#903 <= s_43_4
        fn_state.u_903 = s_43_4;
        // D s_43_6: read-var walkstate.1:struct
        let s_43_6: bool = fn_state.walkstate._1;
        // D s_43_7: cast zx s_43_6 -> bv
        let s_43_7: Bits = Bits::new(s_43_6 as u128, 1u16);
        // C s_43_8: const #1u : u8
        let s_43_8: bool = true;
        // C s_43_9: cast zx s_43_8 -> bv
        let s_43_9: Bits = Bits::new(s_43_8 as u128, 1u16);
        // D s_43_10: cmp-eq s_43_7 s_43_9
        let s_43_10: bool = ((s_43_7) == (s_43_9));
        // N s_43_11: branch s_43_10 b90 b44
        if s_43_10 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#19760 <= s_44_0
        fn_state.gs_19760 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_45_0: read-var gs#19760:u8
        let s_45_0: bool = fn_state.gs_19760;
        // N s_45_1: branch s_45_0 b89 b46
        if s_45_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_46_0: read-var desctype:u32
        let s_46_0: u32 = fn_state.desctype;
        // C s_46_1: const #1u : u32
        let s_46_1: u32 = 1;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // N s_46_3: branch s_46_2 b88 b47
        if s_46_2 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#19794 <= s_47_0
        fn_state.gs_19794 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_48_0: read-var gs#19794:u8
        let s_48_0: bool = fn_state.gs_19794;
        // N s_48_1: branch s_48_0 b87 b49
        if s_48_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#19795 <= s_49_0
        fn_state.gs_19795 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_50_0: read-var gs#19795:u8
        let s_50_0: bool = fn_state.gs_19795;
        // N s_50_1: branch s_50_0 b86 b51
        if s_50_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_51_0: read-var Nshadow#322:i
        let s_51_0: i128 = fn_state.Nshadow_322;
        // D s_51_1: call __id(s_51_0)
        let s_51_1: i128 = u__id(state, tracer, s_51_0);
        // C s_51_2: const #64s : i
        let s_51_2: i128 = 64;
        // D s_51_3: cmp-eq s_51_1 s_51_2
        let s_51_3: bool = ((s_51_1) == (s_51_2));
        // N s_51_4: branch s_51_3 b85 b52
        if s_51_3 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_52_0: read-var Nshadow#322:i
        let s_52_0: i128 = fn_state.Nshadow_322;
        // D s_52_1: call __id(s_52_0)
        let s_52_1: i128 = u__id(state, tracer, s_52_0);
        // C s_52_2: const #128s : i
        let s_52_2: i128 = 128;
        // D s_52_3: cmp-eq s_52_1 s_52_2
        let s_52_3: bool = ((s_52_1) == (s_52_2));
        // D s_52_4: write-var gs#19798 <= s_52_3
        fn_state.gs_19798 = s_52_3;
        // N s_52_5: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_53_0: read-var gs#19798:u8
        let s_53_0: bool = fn_state.gs_19798;
        // N s_53_1: assert s_53_0
        let s_53_1: () = assert!(s_53_0);
        // D s_53_2: read-var walkstate.0:struct
        let s_53_2: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_53_3: write-var ga#15127 <= s_53_2
        fn_state.ga_15127 = s_53_2;
        // D s_53_4: read-var ga#15127.1:struct
        let s_53_4: u32 = fn_state.ga_15127._1;
        // D s_53_5: read-var walkparams:struct
        let s_53_5: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_53_6: read-var regime:u32
        let s_53_6: u32 = fn_state.regime;
        // D s_53_7: read-var descriptor:bv
        let s_53_7: Bits = fn_state.descriptor;
        // D s_53_8: call AArch64_S1AMECFault(s_53_5, s_53_4, s_53_6, s_53_7)
        let s_53_8: bool = AArch64_S1AMECFault(
            state,
            tracer,
            s_53_5,
            s_53_4,
            s_53_6,
            s_53_7,
        );
        // N s_53_9: branch s_53_8 b84 b54
        if s_53_8 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_54_0: read-var u#903.0:struct
        let s_54_0: u64 = fn_state.u_903._0;
        // D s_54_1: read-var walkparams.3:struct
        let s_54_1: bool = fn_state.walkparams._3;
        // D s_54_2: read-var walkparams.28:struct
        let s_54_2: u8 = fn_state.walkparams._28;
        // D s_54_3: read-var walkparams.36:struct
        let s_54_3: u32 = fn_state.walkparams._36;
        // D s_54_4: call AArch64_OAOutOfRange(s_54_0, s_54_1, s_54_2, s_54_3)
        let s_54_4: bool = AArch64_OAOutOfRange(
            state,
            tracer,
            s_54_0,
            s_54_1,
            s_54_2,
            s_54_3,
        );
        // N s_54_5: branch s_54_4 b83 b55
        if s_54_4 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_55_0: const #10s : i
        let s_55_0: i128 = 10;
        // D s_55_1: read-var descriptor:bv
        let s_55_1: Bits = fn_state.descriptor;
        // C s_55_2: const #1u : u64
        let s_55_2: u64 = 1;
        // D s_55_3: bit-extract s_55_1 s_55_0 s_55_2
        let s_55_3: Bits = (Bits::new(
            ((s_55_1) >> (s_55_0)).value(),
            u16::try_from(s_55_2).unwrap(),
        ));
        // D s_55_4: cast reint s_55_3 -> u8
        let s_55_4: bool = ((s_55_3.value()) != 0);
        // C s_55_5: const #0s : i
        let s_55_5: i128 = 0;
        // C s_55_6: const #0u : u64
        let s_55_6: u64 = 0;
        // D s_55_7: cast zx s_55_4 -> u64
        let s_55_7: u64 = (s_55_4 as u64);
        // C s_55_8: const #1u : u64
        let s_55_8: u64 = 1;
        // D s_55_9: and s_55_7 s_55_8
        let s_55_9: u64 = ((s_55_7) & (s_55_8));
        // D s_55_10: cmp-eq s_55_9 s_55_8
        let s_55_10: bool = ((s_55_9) == (s_55_8));
        // D s_55_11: lsl s_55_7 s_55_5
        let s_55_11: u64 = s_55_7 << s_55_5;
        // D s_55_12: or s_55_6 s_55_11
        let s_55_12: u64 = ((s_55_6) | (s_55_11));
        // D s_55_13: cmpl s_55_11
        let s_55_13: u64 = !s_55_11;
        // D s_55_14: and s_55_6 s_55_13
        let s_55_14: u64 = ((s_55_6) & (s_55_13));
        // D s_55_15: select s_55_10 s_55_12 s_55_14
        let s_55_15: u64 = if s_55_10 { s_55_12 } else { s_55_14 };
        // D s_55_16: cast trunc s_55_15 -> u8
        let s_55_16: bool = ((s_55_15) != 0);
        // D s_55_17: cast zx s_55_16 -> bv
        let s_55_17: Bits = Bits::new(s_55_16 as u128, 1u16);
        // C s_55_18: const #0u : u8
        let s_55_18: bool = false;
        // C s_55_19: cast zx s_55_18 -> bv
        let s_55_19: Bits = Bits::new(s_55_18 as u128, 1u16);
        // D s_55_20: cmp-eq s_55_17 s_55_19
        let s_55_20: bool = ((s_55_17) == (s_55_19));
        // N s_55_21: branch s_55_20 b82 b56
        if s_55_20 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#19802 <= s_56_0
        fn_state.gs_19802 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_57_0: read-var gs#19802:u8
        let s_57_0: bool = fn_state.gs_19802;
        // N s_57_1: branch s_57_0 b81 b58
        if s_57_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#19803 <= s_58_0
        fn_state.gs_19803 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_59_0: read-var gs#19803:u8
        let s_59_0: bool = fn_state.gs_19803;
        // N s_59_1: branch s_59_0 b80 b60
        if s_59_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_60_0: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_61_0: read-var fault.16:struct
        let s_61_0: u32 = fn_state.fault._16;
        // C s_61_1: const #0u : u32
        let s_61_1: u32 = 0;
        // D s_61_2: cmp-eq s_61_0 s_61_1
        let s_61_2: bool = ((s_61_0) == (s_61_1));
        // N s_61_3: branch s_61_2 b79 b62
        if s_61_2 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_62_0: const #19088u : u32
        let s_62_0: u32 = 19088;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: bool = {
            let value = state.read_register::<bool>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // N s_62_2: branch s_62_1 b78 b63
        if s_62_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#19761 <= s_63_0
        fn_state.gs_19761 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_64_0: read-var gs#19761:u8
        let s_64_0: bool = fn_state.gs_19761;
        // N s_64_1: branch s_64_0 b77 b65
        if s_64_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#19764 <= s_65_0
        fn_state.gs_19764 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_66_0: read-var gs#19764:u8
        let s_66_0: bool = fn_state.gs_19764;
        // N s_66_1: branch s_66_0 b70 b67
        if s_66_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_67_0: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_68_0: read-var fault:struct
        let s_68_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_68_1: read-var walkaddress:struct
        let s_68_1: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_68_2: read-var walkstate:struct
        let s_68_2: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_68_3: read-var descriptor:bv
        let s_68_3: Bits = fn_state.descriptor;
        // D s_68_4: create-product struct = ["s_68_0", "s_68_1", "s_68_2", "s_68_3"]
        let s_68_4: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_68_0,
            _1: s_68_1,
            _2: s_68_2,
            _3: s_68_3,
        };
        // D s_68_5: write-var return_value <= s_68_4
        fn_state.return_value = s_68_4;
        // N s_68_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_69_0: read-var return_value:struct
        let s_69_0: ProductType4b99944cd5e0b59d = fn_state.return_value;
        // N s_69_1: return s_69_0
        return s_69_0;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_70_0: read-var walkstate.7:struct
        let s_70_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_70_1: write-var ga#15153 <= s_70_0
        fn_state.ga_15153 = s_70_0;
        // D s_70_2: read-var ga#15153.7:struct
        let s_70_2: bool = fn_state.ga_15153._7;
        // D s_70_3: write-var tlbcontext.14 <= s_70_2
        fn_state.tlbcontext._14 = s_70_2;
        // D s_70_4: read-var walkstate.6:struct
        let s_70_4: i128 = fn_state.walkstate._6;
        // D s_70_5: write-var tlbcontext.8 <= s_70_4
        fn_state.tlbcontext._8 = s_70_4;
        // D s_70_6: read-var walkstate.8:struct
        let s_70_6: bool = fn_state.walkstate._8;
        // D s_70_7: write-var tlbcontext.9 <= s_70_6
        fn_state.tlbcontext._9 = s_70_6;
        // D s_70_8: read-var walkparams.3:struct
        let s_70_8: bool = fn_state.walkparams._3;
        // D s_70_9: cast zx s_70_8 -> bv
        let s_70_9: Bits = Bits::new(s_70_8 as u128, 1u16);
        // C s_70_10: const #1u : u8
        let s_70_10: bool = true;
        // C s_70_11: cast zx s_70_10 -> bv
        let s_70_11: Bits = Bits::new(s_70_10 as u128, 1u16);
        // D s_70_12: cmp-eq s_70_9 s_70_11
        let s_70_12: bool = ((s_70_9) == (s_70_11));
        // D s_70_13: write-var tlbcontext.7 <= s_70_12
        fn_state.tlbcontext._7 = s_70_12;
        // D s_70_14: read-var tlbcontext:struct
        let s_70_14: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_70_15: write-var tlbrecord.1 <= s_70_14
        fn_state.tlbrecord._1 = s_70_14;
        // D s_70_16: read-var walkstate:struct
        let s_70_16: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_70_17: write-var tlbrecord.5 <= s_70_16
        fn_state.tlbrecord._5 = s_70_16;
        // D s_70_18: read-var walkparams.3:struct
        let s_70_18: bool = fn_state.walkparams._3;
        // D s_70_19: read-var walkparams.36:struct
        let s_70_19: u32 = fn_state.walkparams._36;
        // D s_70_20: read-var walkstate.6:struct
        let s_70_20: i128 = fn_state.walkstate._6;
        // D s_70_21: call TranslationSize(s_70_18, s_70_19, s_70_20)
        let s_70_21: i128 = TranslationSize(state, tracer, s_70_18, s_70_19, s_70_20);
        // D s_70_22: write-var tlbrecord.0 <= s_70_21
        fn_state.tlbrecord._0 = s_70_21;
        // D s_70_23: read-var walkparams.3:struct
        let s_70_23: bool = fn_state.walkparams._3;
        // D s_70_24: cast zx s_70_23 -> bv
        let s_70_24: Bits = Bits::new(s_70_23 as u128, 1u16);
        // C s_70_25: const #1u : u8
        let s_70_25: bool = true;
        // C s_70_26: cast zx s_70_25 -> bv
        let s_70_26: Bits = Bits::new(s_70_25 as u128, 1u16);
        // D s_70_27: cmp-eq s_70_24 s_70_26
        let s_70_27: bool = ((s_70_24) == (s_70_26));
        // N s_70_28: branch s_70_27 b76 b71
        if s_70_27 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_71_0: read-var Nshadow#322:i
        let s_71_0: i128 = fn_state.Nshadow_322;
        // D s_71_1: call __id(s_71_0)
        let s_71_1: i128 = u__id(state, tracer, s_71_0);
        // C s_71_2: const #63s : i
        let s_71_2: i128 = 63;
        // D s_71_3: cmp-lt s_71_2 s_71_1
        let s_71_3: bool = ((s_71_2) < (s_71_1));
        // N s_71_4: assert s_71_3
        let s_71_4: () = assert!(s_71_3);
        // D s_71_5: read-var tlbrecord:struct
        let s_71_5: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_71_6: write-var tlbrecord <= s_71_5
        fn_state.tlbrecord = s_71_5;
        // D s_71_7: read-var tlbrecord:struct
        let s_71_7: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_71_8: write-var tlbrecord <= s_71_7
        fn_state.tlbrecord = s_71_7;
        // N s_71_9: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_72_0: read-var walkstate.1:struct
        let s_72_0: bool = fn_state.walkstate._1;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // N s_72_5: branch s_72_4 b75 b73
        if s_72_4 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_73_0: const #0s : i
        let s_73_0: i128 = 0;
        // D s_73_1: write-var tlbrecord.2 <= s_73_0
        fn_state.tlbrecord._2 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_74_0: read-var tlbrecord:struct
        let s_74_0: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_74_1: call S1TLBCache(s_74_0)
        let s_74_1: () = S1TLBCache(state, tracer, s_74_0);
        // N s_74_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_75_0: read-var walkparams.3:struct
        let s_75_0: bool = fn_state.walkparams._3;
        // D s_75_1: read-var walkparams.36:struct
        let s_75_1: u32 = fn_state.walkparams._36;
        // D s_75_2: read-var walkstate.6:struct
        let s_75_2: i128 = fn_state.walkstate._6;
        // D s_75_3: call ContiguousSize(s_75_0, s_75_1, s_75_2)
        let s_75_3: i128 = ContiguousSize(state, tracer, s_75_0, s_75_1, s_75_2);
        // D s_75_4: write-var tlbrecord.2 <= s_75_3
        fn_state.tlbrecord._2 = s_75_3;
        // N s_75_5: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_76_0: read-var Nshadow#322:i
        let s_76_0: i128 = fn_state.Nshadow_322;
        // D s_76_1: call __id(s_76_0)
        let s_76_1: i128 = u__id(state, tracer, s_76_0);
        // C s_76_2: const #127s : i
        let s_76_2: i128 = 127;
        // D s_76_3: cmp-lt s_76_2 s_76_1
        let s_76_3: bool = ((s_76_2) < (s_76_1));
        // N s_76_4: assert s_76_3
        let s_76_4: () = assert!(s_76_3);
        // C s_76_5: const #0s : i
        let s_76_5: i128 = 0;
        // D s_76_6: read-var descriptor:bv
        let s_76_6: Bits = fn_state.descriptor;
        // C s_76_7: const #1s : i64
        let s_76_7: i64 = 1;
        // C s_76_8: cast zx s_76_7 -> i
        let s_76_8: i128 = (i128::try_from(s_76_7).unwrap());
        // C s_76_9: const #127s : i
        let s_76_9: i128 = 127;
        // C s_76_10: add s_76_9 s_76_8
        let s_76_10: i128 = (s_76_9 + s_76_8);
        // D s_76_11: bit-extract s_76_6 s_76_5 s_76_10
        let s_76_11: Bits = (Bits::new(
            ((s_76_6) >> (s_76_5)).value(),
            u16::try_from(s_76_10).unwrap(),
        ));
        // D s_76_12: cast reint s_76_11 -> u128
        let s_76_12: u128 = (s_76_11.value() as u128);
        // D s_76_13: write-var tlbrecord.3 <= s_76_12
        fn_state.tlbrecord._3 = s_76_12;
        // N s_76_14: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_77_0: const #10s : i
        let s_77_0: i128 = 10;
        // D s_77_1: read-var descriptor:bv
        let s_77_1: Bits = fn_state.descriptor;
        // C s_77_2: const #1u : u64
        let s_77_2: u64 = 1;
        // D s_77_3: bit-extract s_77_1 s_77_0 s_77_2
        let s_77_3: Bits = (Bits::new(
            ((s_77_1) >> (s_77_0)).value(),
            u16::try_from(s_77_2).unwrap(),
        ));
        // D s_77_4: cast reint s_77_3 -> u8
        let s_77_4: bool = ((s_77_3.value()) != 0);
        // C s_77_5: const #0s : i
        let s_77_5: i128 = 0;
        // C s_77_6: const #0u : u64
        let s_77_6: u64 = 0;
        // D s_77_7: cast zx s_77_4 -> u64
        let s_77_7: u64 = (s_77_4 as u64);
        // C s_77_8: const #1u : u64
        let s_77_8: u64 = 1;
        // D s_77_9: and s_77_7 s_77_8
        let s_77_9: u64 = ((s_77_7) & (s_77_8));
        // D s_77_10: cmp-eq s_77_9 s_77_8
        let s_77_10: bool = ((s_77_9) == (s_77_8));
        // D s_77_11: lsl s_77_7 s_77_5
        let s_77_11: u64 = s_77_7 << s_77_5;
        // D s_77_12: or s_77_6 s_77_11
        let s_77_12: u64 = ((s_77_6) | (s_77_11));
        // D s_77_13: cmpl s_77_11
        let s_77_13: u64 = !s_77_11;
        // D s_77_14: and s_77_6 s_77_13
        let s_77_14: u64 = ((s_77_6) & (s_77_13));
        // D s_77_15: select s_77_10 s_77_12 s_77_14
        let s_77_15: u64 = if s_77_10 { s_77_12 } else { s_77_14 };
        // D s_77_16: cast trunc s_77_15 -> u8
        let s_77_16: bool = ((s_77_15) != 0);
        // D s_77_17: cast zx s_77_16 -> bv
        let s_77_17: Bits = Bits::new(s_77_16 as u128, 1u16);
        // C s_77_18: const #1u : u8
        let s_77_18: bool = true;
        // C s_77_19: cast zx s_77_18 -> bv
        let s_77_19: Bits = Bits::new(s_77_18 as u128, 1u16);
        // D s_77_20: cmp-eq s_77_17 s_77_19
        let s_77_20: bool = ((s_77_17) == (s_77_19));
        // D s_77_21: write-var gs#19764 <= s_77_20
        fn_state.gs_19764 = s_77_20;
        // N s_77_22: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_78_0: read-var fault.16:struct
        let s_78_0: u32 = fn_state.fault._16;
        // C s_78_1: const #0u : u32
        let s_78_1: u32 = 0;
        // D s_78_2: cmp-eq s_78_0 s_78_1
        let s_78_2: bool = ((s_78_0) == (s_78_1));
        // D s_78_3: write-var gs#19761 <= s_78_2
        fn_state.gs_19761 = s_78_2;
        // N s_78_4: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call __UNKNOWN_AddressDescriptor(s_79_0)
        let s_79_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_79_0,
        );
        // C s_79_2: const #() : ()
        let s_79_2: () = ();
        // S s_79_3: call __UNKNOWN_TTWState(s_79_2)
        let s_79_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_79_2,
        );
        // D s_79_4: read-var Nshadow#322:i
        let s_79_4: i128 = fn_state.Nshadow_322;
        // D s_79_5: call __UNKNOWN_bits(s_79_4)
        let s_79_5: Bits = u__UNKNOWN_bits(state, tracer, s_79_4);
        // D s_79_6: read-var fault:struct
        let s_79_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_79_7: create-product struct = ["s_79_6", "s_79_1", "s_79_3", "s_79_5"]
        let s_79_7: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_79_6,
            _1: s_79_1,
            _2: s_79_3,
            _3: s_79_5,
        };
        // D s_79_8: write-var return_value <= s_79_7
        fn_state.return_value = s_79_7;
        // N s_79_9: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_80_0: const #1u : u32
        let s_80_0: u32 = 1;
        // D s_80_1: write-var fault.16 <= s_80_0
        fn_state.fault._16 = s_80_0;
        // N s_80_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // D s_81_1: write-var gs#19803 <= s_81_0
        fn_state.gs_19803 = s_81_0;
        // N s_81_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_82_0: read-var walkparams.12:struct
        let s_82_0: bool = fn_state.walkparams._12;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 1u16);
        // C s_82_2: const #0u : u8
        let s_82_2: bool = false;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: write-var gs#19802 <= s_82_4
        fn_state.gs_19802 = s_82_4;
        // N s_82_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_83_0: const #7u : u32
        let s_83_0: u32 = 7;
        // D s_83_1: write-var fault.16 <= s_83_0
        fn_state.fault._16 = s_83_0;
        // N s_83_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_84_0: const #6u : u32
        let s_84_0: u32 = 6;
        // D s_84_1: write-var fault.16 <= s_84_0
        fn_state.fault._16 = s_84_0;
        // N s_84_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_85_0: const #1u : u8
        let s_85_0: bool = true;
        // D s_85_1: write-var gs#19798 <= s_85_0
        fn_state.gs_19798 = s_85_0;
        // N s_85_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_86_0: const #6u : u32
        let s_86_0: u32 = 6;
        // D s_86_1: write-var fault.16 <= s_86_0
        fn_state.fault._16 = s_86_0;
        // N s_86_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_87_0: read-var walkparams.3:struct
        let s_87_0: bool = fn_state.walkparams._3;
        // D s_87_1: read-var descriptor:bv
        let s_87_1: Bits = fn_state.descriptor;
        // D s_87_2: call AArch64_BlocknTFaults(s_87_0, s_87_1)
        let s_87_2: bool = AArch64_BlocknTFaults(state, tracer, s_87_0, s_87_1);
        // D s_87_3: write-var gs#19795 <= s_87_2
        fn_state.gs_19795 = s_87_2;
        // N s_87_4: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_88_0: read-var walkstate.6:struct
        let s_88_0: i128 = fn_state.walkstate._6;
        // C s_88_1: const #800u : u32
        let s_88_1: u32 = 800;
        // D s_88_2: read-reg s_88_1:i64
        let s_88_2: i64 = {
            let value = state.read_register::<i64>(s_88_1 as isize);
            tracer.read_register(s_88_1 as isize, value);
            value
        };
        // D s_88_3: cast zx s_88_2 -> i
        let s_88_3: i128 = (i128::try_from(s_88_2).unwrap());
        // D s_88_4: cmp-lt s_88_0 s_88_3
        let s_88_4: bool = ((s_88_0) < (s_88_3));
        // D s_88_5: write-var gs#19794 <= s_88_4
        fn_state.gs_19794 = s_88_4;
        // N s_88_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_89_0: const #6u : u32
        let s_89_0: u32 = 6;
        // D s_89_1: write-var fault.16 <= s_89_0
        fn_state.fault._16 = s_89_0;
        // N s_89_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_90_0: read-var walkparams.3:struct
        let s_90_0: bool = fn_state.walkparams._3;
        // D s_90_1: read-var walkparams.37:struct
        let s_90_1: u8 = fn_state.walkparams._37;
        // D s_90_2: read-var walkparams.36:struct
        let s_90_2: u32 = fn_state.walkparams._36;
        // D s_90_3: read-var walkstate.6:struct
        let s_90_3: i128 = fn_state.walkstate._6;
        // D s_90_4: call AArch64_ContiguousBitFaults(s_90_0, s_90_1, s_90_2, s_90_3)
        let s_90_4: bool = AArch64_ContiguousBitFaults(
            state,
            tracer,
            s_90_0,
            s_90_1,
            s_90_2,
            s_90_3,
        );
        // D s_90_5: write-var gs#19760 <= s_90_4
        fn_state.gs_19760 = s_90_4;
        // N s_90_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_91_0: read-var accdesc:struct
        let s_91_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_91_1: call CreateAccDescTTEUpdate(s_91_0)
        let s_91_1: ProductType9878976b5bcce9c9 = CreateAccDescTTEUpdate(
            state,
            tracer,
            s_91_0,
        );
        // D s_91_2: write-var descaccess <= s_91_1
        fn_state.descaccess = s_91_1;
        // D s_91_3: read-var regime:u32
        let s_91_3: u32 = fn_state.regime;
        // C s_91_4: const #4u : u32
        let s_91_4: u32 = 4;
        // D s_91_5: cmp-eq s_91_3 s_91_4
        let s_91_5: bool = ((s_91_3) == (s_91_4));
        // N s_91_6: branch s_91_5 b104 b92
        if s_91_5 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#19727 <= s_92_0
        fn_state.gs_19727 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_93_0: read-var gs#19727:u8
        let s_93_0: bool = fn_state.gs_19727;
        // N s_93_1: branch s_93_0 b101 b94
        if s_93_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_94_0: read-var walkaddress:struct
        let s_94_0: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_94_1: write-var descpaddr <= s_94_0
        fn_state.descpaddr = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_95_0: read-var Nshadow#322:i
        let s_95_0: i128 = fn_state.Nshadow_322;
        // D s_95_1: call __id(s_95_0)
        let s_95_1: i128 = u__id(state, tracer, s_95_0);
        // C s_95_2: const #64s : i
        let s_95_2: i128 = 64;
        // D s_95_3: cmp-eq s_95_1 s_95_2
        let s_95_3: bool = ((s_95_1) == (s_95_2));
        // N s_95_4: branch s_95_3 b100 b96
        if s_95_3 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_96_0: read-var Nshadow#322:i
        let s_96_0: i128 = fn_state.Nshadow_322;
        // D s_96_1: call __id(s_96_0)
        let s_96_1: i128 = u__id(state, tracer, s_96_0);
        // C s_96_2: const #128s : i
        let s_96_2: i128 = 128;
        // D s_96_3: cmp-eq s_96_1 s_96_2
        let s_96_3: bool = ((s_96_1) == (s_96_2));
        // D s_96_4: write-var gs#19730 <= s_96_3
        fn_state.gs_19730 = s_96_3;
        // N s_96_5: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_97_0: read-var gs#19730:u8
        let s_97_0: bool = fn_state.gs_19730;
        // N s_97_1: assert s_97_0
        let s_97_1: () = assert!(s_97_0);
        // D s_97_2: read-var walkparams.9:struct
        let s_97_2: bool = fn_state.walkparams._9;
        // D s_97_3: read-var fault:struct
        let s_97_3: ProductType1d757adad216cdef = fn_state.fault;
        // D s_97_4: read-var descriptor:bv
        let s_97_4: Bits = fn_state.descriptor;
        // D s_97_5: read-var new_descriptor:bv
        let s_97_5: Bits = fn_state.new_descriptor;
        // D s_97_6: read-var descaccess:struct
        let s_97_6: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_97_7: read-var descpaddr:struct
        let s_97_7: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_97_8: read-var translation_info:struct
        let s_97_8: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_97_9: call AArch64_MemSwapTableDesc(s_97_3, s_97_4, s_97_5, s_97_2, s_97_6, s_97_7, s_97_8)
        let s_97_9: ProductTypeb4cea7287e2eb9d6 = AArch64_MemSwapTableDesc(
            state,
            tracer,
            s_97_3,
            s_97_4,
            s_97_5,
            s_97_2,
            s_97_6,
            s_97_7,
            s_97_8,
        );
        // D s_97_10: write-var ga#15093 <= s_97_9
        fn_state.ga_15093 = s_97_9;
        // D s_97_11: read-var ga#15093.0:struct
        let s_97_11: ProductType1d757adad216cdef = fn_state.ga_15093._0;
        // D s_97_12: read-var ga#15093.1:struct
        let s_97_12: Bits = fn_state.ga_15093._1;
        // D s_97_13: write-var fault <= s_97_11
        fn_state.fault = s_97_11;
        // D s_97_14: write-var descriptor <= s_97_12
        fn_state.descriptor = s_97_12;
        // D s_97_15: read-var fault.16:struct
        let s_97_15: u32 = fn_state.fault._16;
        // C s_97_16: const #0u : u32
        let s_97_16: u32 = 0;
        // D s_97_17: cmp-eq s_97_15 s_97_16
        let s_97_17: bool = ((s_97_15) == (s_97_16));
        // N s_97_18: branch s_97_17 b99 b98
        if s_97_17 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_98_0: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_99_0: const #() : ()
        let s_99_0: () = ();
        // S s_99_1: call __UNKNOWN_AddressDescriptor(s_99_0)
        let s_99_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_99_0,
        );
        // C s_99_2: const #() : ()
        let s_99_2: () = ();
        // S s_99_3: call __UNKNOWN_TTWState(s_99_2)
        let s_99_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_99_2,
        );
        // D s_99_4: read-var Nshadow#322:i
        let s_99_4: i128 = fn_state.Nshadow_322;
        // D s_99_5: cast reint s_99_4 -> i64
        let s_99_5: i64 = (s_99_4 as i64);
        // D s_99_6: cast zx s_99_5 -> i
        let s_99_6: i128 = (i128::try_from(s_99_5).unwrap());
        // D s_99_7: call __UNKNOWN_bits(s_99_6)
        let s_99_7: Bits = u__UNKNOWN_bits(state, tracer, s_99_6);
        // D s_99_8: read-var fault:struct
        let s_99_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_99_9: create-product struct = ["s_99_8", "s_99_1", "s_99_3", "s_99_7"]
        let s_99_9: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_99_8,
            _1: s_99_1,
            _2: s_99_3,
            _3: s_99_7,
        };
        // D s_99_10: write-var return_value <= s_99_9
        fn_state.return_value = s_99_9;
        // N s_99_11: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // D s_100_1: write-var gs#19730 <= s_100_0
        fn_state.gs_19730 = s_100_0;
        // N s_100_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_101_0: read-var walkstate.6:struct
        let s_101_0: i128 = fn_state.walkstate._6;
        // D s_101_1: create-sum enum = 1:"s_101_0"
        let s_101_1: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_1(s_101_0);
        // D s_101_2: read-var fault:struct
        let s_101_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_101_3: read-var walkaddress:struct
        let s_101_3: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // C s_101_4: const #1u : u8
        let s_101_4: bool = true;
        // C s_101_5: const #1u : u8
        let s_101_5: bool = true;
        // D s_101_6: read-var descaccess:struct
        let s_101_6: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_101_7: call AArch64_S2Translate(s_101_2, s_101_3, s_101_4, s_101_1, s_101_5, s_101_6)
        let s_101_7: ProductTypedc31059ca7e2391c = AArch64_S2Translate(
            state,
            tracer,
            s_101_2,
            s_101_3,
            s_101_4,
            s_101_1,
            s_101_5,
            s_101_6,
        );
        // D s_101_8: write-var ga#15080 <= s_101_7
        fn_state.ga_15080 = s_101_7;
        // D s_101_9: read-var ga#15080.0:struct
        let s_101_9: ProductType1d757adad216cdef = fn_state.ga_15080._0;
        // D s_101_10: read-var ga#15080.1:struct
        let s_101_10: ProductTypece7c66ccb2cab13e = fn_state.ga_15080._1;
        // D s_101_11: write-var s2fault <= s_101_9
        fn_state.s2fault = s_101_9;
        // D s_101_12: write-var descpaddr <= s_101_10
        fn_state.descpaddr = s_101_10;
        // D s_101_13: read-var s2fault.16:struct
        let s_101_13: u32 = fn_state.s2fault._16;
        // C s_101_14: const #0u : u32
        let s_101_14: u32 = 0;
        // D s_101_15: cmp-eq s_101_13 s_101_14
        let s_101_15: bool = ((s_101_13) == (s_101_14));
        // N s_101_16: branch s_101_15 b103 b102
        if s_101_15 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_102_0: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call __UNKNOWN_AddressDescriptor(s_103_0)
        let s_103_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_103_0,
        );
        // C s_103_2: const #() : ()
        let s_103_2: () = ();
        // S s_103_3: call __UNKNOWN_TTWState(s_103_2)
        let s_103_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_103_2,
        );
        // D s_103_4: read-var Nshadow#322:i
        let s_103_4: i128 = fn_state.Nshadow_322;
        // D s_103_5: call __UNKNOWN_bits(s_103_4)
        let s_103_5: Bits = u__UNKNOWN_bits(state, tracer, s_103_4);
        // D s_103_6: read-var s2fault:struct
        let s_103_6: ProductType1d757adad216cdef = fn_state.s2fault;
        // D s_103_7: create-product struct = ["s_103_6", "s_103_1", "s_103_3", "s_103_5"]
        let s_103_7: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_103_6,
            _1: s_103_1,
            _2: s_103_3,
            _3: s_103_5,
        };
        // D s_103_8: write-var return_value <= s_103_7
        fn_state.return_value = s_103_7;
        // N s_103_9: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_104_0: const #() : ()
        let s_104_0: () = ();
        // S s_104_1: call EL2Enabled(s_104_0)
        let s_104_1: bool = EL2Enabled(state, tracer, s_104_0);
        // D s_104_2: write-var gs#19727 <= s_104_1
        fn_state.gs_19727 = s_104_1;
        // N s_104_3: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_105_0: const #6u : u32
        let s_105_0: u32 = 6;
        // D s_105_1: write-var fault.16 <= s_105_0
        fn_state.fault._16 = s_105_0;
        // C s_105_2: const #() : ()
        let s_105_2: () = ();
        // S s_105_3: call __UNKNOWN_AddressDescriptor(s_105_2)
        let s_105_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_105_2,
        );
        // C s_105_4: const #() : ()
        let s_105_4: () = ();
        // S s_105_5: call __UNKNOWN_TTWState(s_105_4)
        let s_105_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_105_4,
        );
        // D s_105_6: read-var Nshadow#322:i
        let s_105_6: i128 = fn_state.Nshadow_322;
        // D s_105_7: call __UNKNOWN_bits(s_105_6)
        let s_105_7: Bits = u__UNKNOWN_bits(state, tracer, s_105_6);
        // D s_105_8: read-var fault:struct
        let s_105_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_105_9: create-product struct = ["s_105_8", "s_105_3", "s_105_5", "s_105_7"]
        let s_105_9: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_105_8,
            _1: s_105_3,
            _2: s_105_5,
            _3: s_105_7,
        };
        // D s_105_10: write-var return_value <= s_105_9
        fn_state.return_value = s_105_9;
        // N s_105_11: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_106_0: read-var walkparams.3:struct
        let s_106_0: bool = fn_state.walkparams._3;
        // D s_106_1: read-var descriptor:bv
        let s_106_1: Bits = fn_state.descriptor;
        // D s_106_2: call AArch64_BlocknTFaults(s_106_0, s_106_1)
        let s_106_2: bool = AArch64_BlocknTFaults(state, tracer, s_106_0, s_106_1);
        // D s_106_3: write-var gs#19706 <= s_106_2
        fn_state.gs_19706 = s_106_2;
        // N s_106_4: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_107_0: read-var skl:u8
        let s_107_0: u8 = fn_state.skl;
        // D s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 2u16);
        // C s_107_2: const #0u : u8
        let s_107_2: u8 = 0;
        // C s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 2u16);
        // D s_107_4: cmp-ne s_107_1 s_107_3
        let s_107_4: bool = ((s_107_1) != (s_107_3));
        // D s_107_5: write-var gs#19705 <= s_107_4
        fn_state.gs_19705 = s_107_4;
        // N s_107_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_108_0: read-var Nshadow#322:i
        let s_108_0: i128 = fn_state.Nshadow_322;
        // D s_108_1: call __id(s_108_0)
        let s_108_1: i128 = u__id(state, tracer, s_108_0);
        // C s_108_2: const #10s : i
        let s_108_2: i128 = 10;
        // D s_108_3: cmp-lt s_108_2 s_108_1
        let s_108_3: bool = ((s_108_2) < (s_108_1));
        // N s_108_4: assert s_108_3
        let s_108_4: () = assert!(s_108_3);
        // C s_108_5: const #10s : i
        let s_108_5: i128 = 10;
        // D s_108_6: read-var new_descriptor:bv
        let s_108_6: Bits = fn_state.new_descriptor;
        // C s_108_7: const #1u : u64
        let s_108_7: u64 = 1;
        // D s_108_8: bit-insert s_108_6 s_108_6 s_108_5 s_108_7
        let s_108_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_108_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_108_6.length(),
            );
            (s_108_6 & mask) | (s_108_6 << s_108_5)
        };
        // D s_108_9: write-var new_descriptor <= s_108_8
        fn_state.new_descriptor = s_108_8;
        // N s_108_10: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_109_0: const #7u : u32
        let s_109_0: u32 = 7;
        // D s_109_1: write-var fault.16 <= s_109_0
        fn_state.fault._16 = s_109_0;
        // C s_109_2: const #() : ()
        let s_109_2: () = ();
        // S s_109_3: call __UNKNOWN_AddressDescriptor(s_109_2)
        let s_109_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_109_2,
        );
        // C s_109_4: const #() : ()
        let s_109_4: () = ();
        // S s_109_5: call __UNKNOWN_TTWState(s_109_4)
        let s_109_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_109_4,
        );
        // D s_109_6: read-var Nshadow#322:i
        let s_109_6: i128 = fn_state.Nshadow_322;
        // D s_109_7: call __UNKNOWN_bits(s_109_6)
        let s_109_7: Bits = u__UNKNOWN_bits(state, tracer, s_109_6);
        // D s_109_8: read-var fault:struct
        let s_109_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_109_9: create-product struct = ["s_109_8", "s_109_3", "s_109_5", "s_109_7"]
        let s_109_9: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_109_8,
            _1: s_109_3,
            _2: s_109_5,
            _3: s_109_7,
        };
        // D s_109_10: write-var return_value <= s_109_9
        fn_state.return_value = s_109_9;
        // N s_109_11: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_110_0: read-var Nshadow#322:i
        let s_110_0: i128 = fn_state.Nshadow_322;
        // D s_110_1: call __id(s_110_0)
        let s_110_1: i128 = u__id(state, tracer, s_110_0);
        // C s_110_2: const #110s : i
        let s_110_2: i128 = 110;
        // D s_110_3: cmp-lt s_110_2 s_110_1
        let s_110_3: bool = ((s_110_2) < (s_110_1));
        // N s_110_4: assert s_110_3
        let s_110_4: () = assert!(s_110_3);
        // C s_110_5: const #109s : i
        let s_110_5: i128 = 109;
        // D s_110_6: read-var descriptor:bv
        let s_110_6: Bits = fn_state.descriptor;
        // C s_110_7: const #1s : i64
        let s_110_7: i64 = 1;
        // C s_110_8: cast zx s_110_7 -> i
        let s_110_8: i128 = (i128::try_from(s_110_7).unwrap());
        // C s_110_9: const #1s : i
        let s_110_9: i128 = 1;
        // C s_110_10: add s_110_9 s_110_8
        let s_110_10: i128 = (s_110_9 + s_110_8);
        // D s_110_11: bit-extract s_110_6 s_110_5 s_110_10
        let s_110_11: Bits = (Bits::new(
            ((s_110_6) >> (s_110_5)).value(),
            u16::try_from(s_110_10).unwrap(),
        ));
        // D s_110_12: cast reint s_110_11 -> u8
        let s_110_12: u8 = (s_110_11.value() as u8);
        // D s_110_13: write-var skl <= s_110_12
        fn_state.skl = s_110_12;
        // N s_110_14: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_111_0: const #1u : u32
        let s_111_0: u32 = 1;
        // D s_111_1: read-var desctype:u32
        let s_111_1: u32 = fn_state.desctype;
        // D s_111_2: cmp-eq s_111_0 s_111_1
        let s_111_2: bool = ((s_111_0) == (s_111_1));
        // D s_111_3: not s_111_2
        let s_111_3: bool = !s_111_2;
        // N s_111_4: branch s_111_3 b113 b112
        if s_111_3 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_112_0: read-var accdesc.25:struct
        let s_112_0: u32 = fn_state.accdesc._25;
        // D s_112_1: read-var walkstate:struct
        let s_112_1: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_112_2: read-var s2fs1mro:u8
        let s_112_2: bool = fn_state.s2fs1mro;
        // D s_112_3: read-var regime:u32
        let s_112_3: u32 = fn_state.regime;
        // D s_112_4: read-var walkparams:struct
        let s_112_4: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_112_5: read-var descriptor:bv
        let s_112_5: Bits = fn_state.descriptor;
        // D s_112_6: call AArch64_S1NextWalkStateLeaf(s_112_1, s_112_2, s_112_3, s_112_0, s_112_4, s_112_5)
        let s_112_6: ProductType96e7acababe246a1 = AArch64_S1NextWalkStateLeaf(
            state,
            tracer,
            s_112_1,
            s_112_2,
            s_112_3,
            s_112_0,
            s_112_4,
            s_112_5,
        );
        // D s_112_7: write-var walkstate <= s_112_6
        fn_state.walkstate = s_112_6;
        // N s_112_8: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_113_0: const #2u : u32
        let s_113_0: u32 = 2;
        // D s_113_1: read-var desctype:u32
        let s_113_1: u32 = fn_state.desctype;
        // D s_113_2: cmp-eq s_113_0 s_113_1
        let s_113_2: bool = ((s_113_0) == (s_113_1));
        // D s_113_3: not s_113_2
        let s_113_3: bool = !s_113_2;
        // N s_113_4: branch s_113_3 b115 b114
        if s_113_3 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_114_0: const #6u : u32
        let s_114_0: u32 = 6;
        // D s_114_1: write-var fault.16 <= s_114_0
        fn_state.fault._16 = s_114_0;
        // C s_114_2: const #() : ()
        let s_114_2: () = ();
        // S s_114_3: call __UNKNOWN_AddressDescriptor(s_114_2)
        let s_114_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_114_2,
        );
        // C s_114_4: const #() : ()
        let s_114_4: () = ();
        // S s_114_5: call __UNKNOWN_TTWState(s_114_4)
        let s_114_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_114_4,
        );
        // D s_114_6: read-var Nshadow#322:i
        let s_114_6: i128 = fn_state.Nshadow_322;
        // D s_114_7: call __UNKNOWN_bits(s_114_6)
        let s_114_7: Bits = u__UNKNOWN_bits(state, tracer, s_114_6);
        // D s_114_8: read-var fault:struct
        let s_114_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_114_9: create-product struct = ["s_114_8", "s_114_3", "s_114_5", "s_114_7"]
        let s_114_9: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_114_8,
            _1: s_114_3,
            _2: s_114_5,
            _3: s_114_7,
        };
        // D s_114_10: write-var return_value <= s_114_9
        fn_state.return_value = s_114_9;
        // N s_114_11: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_115_0: const #() : ()
        let s_115_0: () = ();
        // S s_115_1: call Unreachable(s_115_0)
        let s_115_1: () = Unreachable(state, tracer, s_115_0);
        // N s_115_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call __UNKNOWN_AddressDescriptor(s_116_0)
        let s_116_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_116_0,
        );
        // C s_116_2: const #() : ()
        let s_116_2: () = ();
        // S s_116_3: call __UNKNOWN_TTWState(s_116_2)
        let s_116_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_116_2,
        );
        // D s_116_4: read-var Nshadow#322:i
        let s_116_4: i128 = fn_state.Nshadow_322;
        // D s_116_5: call __UNKNOWN_bits(s_116_4)
        let s_116_5: Bits = u__UNKNOWN_bits(state, tracer, s_116_4);
        // D s_116_6: read-var fault:struct
        let s_116_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_116_7: create-product struct = ["s_116_6", "s_116_1", "s_116_3", "s_116_5"]
        let s_116_7: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_116_6,
            _1: s_116_1,
            _2: s_116_3,
            _3: s_116_5,
        };
        // D s_116_8: write-var return_value <= s_116_7
        fn_state.return_value = s_116_7;
        // N s_116_9: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_117_0: const #1u : u8
        let s_117_0: bool = true;
        // D s_117_1: write-var gs#19677 <= s_117_0
        fn_state.gs_19677 = s_117_0;
        // N s_117_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_118_0: const #1u : u8
        let s_118_0: bool = true;
        // D s_118_1: write-var gs#19675 <= s_118_0
        fn_state.gs_19675 = s_118_0;
        // N s_118_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_119_0: read-var walkstate.6:struct
        let s_119_0: i128 = fn_state.walkstate._6;
        // D s_119_1: create-sum enum = 1:"s_119_0"
        let s_119_1: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_1(s_119_0);
        // D s_119_2: read-var fault:struct
        let s_119_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_119_3: read-var walkaddress:struct
        let s_119_3: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // C s_119_4: const #1u : u8
        let s_119_4: bool = true;
        // C s_119_5: const #1u : u8
        let s_119_5: bool = true;
        // D s_119_6: read-var walkaccess:struct
        let s_119_6: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_119_7: call AArch64_S2Translate(s_119_2, s_119_3, s_119_4, s_119_1, s_119_5, s_119_6)
        let s_119_7: ProductTypedc31059ca7e2391c = AArch64_S2Translate(
            state,
            tracer,
            s_119_2,
            s_119_3,
            s_119_4,
            s_119_1,
            s_119_5,
            s_119_6,
        );
        // D s_119_8: write-var ga#14999 <= s_119_7
        fn_state.ga_14999 = s_119_7;
        // D s_119_9: read-var ga#14999.0:struct
        let s_119_9: ProductType1d757adad216cdef = fn_state.ga_14999._0;
        // D s_119_10: read-var ga#14999.1:struct
        let s_119_10: ProductTypece7c66ccb2cab13e = fn_state.ga_14999._1;
        // D s_119_11: write-var s2fault <= s_119_9
        fn_state.s2fault = s_119_9;
        // D s_119_12: write-var s2walkaddress <= s_119_10
        fn_state.s2walkaddress = s_119_10;
        // D s_119_13: read-var s2fault.16:struct
        let s_119_13: u32 = fn_state.s2fault._16;
        // C s_119_14: const #0u : u32
        let s_119_14: u32 = 0;
        // D s_119_15: cmp-eq s_119_13 s_119_14
        let s_119_15: bool = ((s_119_13) == (s_119_14));
        // N s_119_16: branch s_119_15 b127 b120
        if s_119_15 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_120_0: read-var s2walkaddress.5:struct
        let s_120_0: bool = fn_state.s2walkaddress._5;
        // D s_120_1: write-var s2fs1mro <= s_120_0
        fn_state.s2fs1mro = s_120_0;
        // D s_120_2: read-var Nshadow#322:i
        let s_120_2: i128 = fn_state.Nshadow_322;
        // D s_120_3: call __id(s_120_2)
        let s_120_3: i128 = u__id(state, tracer, s_120_2);
        // C s_120_4: const #32s : i
        let s_120_4: i128 = 32;
        // D s_120_5: cmp-eq s_120_3 s_120_4
        let s_120_5: bool = ((s_120_3) == (s_120_4));
        // N s_120_6: branch s_120_5 b126 b121
        if s_120_5 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_121_0: read-var Nshadow#322:i
        let s_121_0: i128 = fn_state.Nshadow_322;
        // D s_121_1: call __id(s_121_0)
        let s_121_1: i128 = u__id(state, tracer, s_121_0);
        // C s_121_2: const #64s : i
        let s_121_2: i128 = 64;
        // D s_121_3: cmp-eq s_121_1 s_121_2
        let s_121_3: bool = ((s_121_1) == (s_121_2));
        // D s_121_4: write-var gs#19687 <= s_121_3
        fn_state.gs_19687 = s_121_3;
        // N s_121_5: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_122_0: read-var gs#19687:u8
        let s_122_0: bool = fn_state.gs_19687;
        // N s_122_1: branch s_122_0 b125 b123
        if s_122_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_123_0: read-var Nshadow#322:i
        let s_123_0: i128 = fn_state.Nshadow_322;
        // D s_123_1: call __id(s_123_0)
        let s_123_1: i128 = u__id(state, tracer, s_123_0);
        // C s_123_2: const #128s : i
        let s_123_2: i128 = 128;
        // D s_123_3: cmp-eq s_123_1 s_123_2
        let s_123_3: bool = ((s_123_1) == (s_123_2));
        // D s_123_4: write-var gs#19689 <= s_123_3
        fn_state.gs_19689 = s_123_3;
        // N s_123_5: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_124_0: read-var gs#19689:u8
        let s_124_0: bool = fn_state.gs_19689;
        // N s_124_1: assert s_124_0
        let s_124_1: () = assert!(s_124_0);
        // D s_124_2: read-var walkparams.9:struct
        let s_124_2: bool = fn_state.walkparams._9;
        // D s_124_3: read-var Nshadow#322:i
        let s_124_3: i128 = fn_state.Nshadow_322;
        // D s_124_4: cast reint s_124_3 -> i64
        let s_124_4: i64 = (s_124_3 as i64);
        // D s_124_5: read-var s2walkaddress:struct
        let s_124_5: ProductTypece7c66ccb2cab13e = fn_state.s2walkaddress;
        // D s_124_6: read-var walkaccess:struct
        let s_124_6: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_124_7: read-var fault:struct
        let s_124_7: ProductType1d757adad216cdef = fn_state.fault;
        // D s_124_8: read-var translation_info:struct
        let s_124_8: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_124_9: call FetchDescriptor(s_124_2, s_124_5, s_124_6, s_124_7, s_124_4, s_124_8)
        let s_124_9: ProductTypeb4cea7287e2eb9d6 = FetchDescriptor(
            state,
            tracer,
            s_124_2,
            s_124_5,
            s_124_6,
            s_124_7,
            s_124_4,
            s_124_8,
        );
        // D s_124_10: write-var ga#15015 <= s_124_9
        fn_state.ga_15015 = s_124_9;
        // D s_124_11: read-var ga#15015.0:struct
        let s_124_11: ProductType1d757adad216cdef = fn_state.ga_15015._0;
        // D s_124_12: read-var ga#15015.1:struct
        let s_124_12: Bits = fn_state.ga_15015._1;
        // D s_124_13: write-var fault <= s_124_11
        fn_state.fault = s_124_11;
        // D s_124_14: write-var descriptor <= s_124_12
        fn_state.descriptor = s_124_12;
        // N s_124_15: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_125_0: const #1u : u8
        let s_125_0: bool = true;
        // D s_125_1: write-var gs#19689 <= s_125_0
        fn_state.gs_19689 = s_125_0;
        // N s_125_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_126_0: const #1u : u8
        let s_126_0: bool = true;
        // D s_126_1: write-var gs#19687 <= s_126_0
        fn_state.gs_19687 = s_126_0;
        // N s_126_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_127_0: const #() : ()
        let s_127_0: () = ();
        // S s_127_1: call __UNKNOWN_AddressDescriptor(s_127_0)
        let s_127_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_127_0,
        );
        // C s_127_2: const #() : ()
        let s_127_2: () = ();
        // S s_127_3: call __UNKNOWN_TTWState(s_127_2)
        let s_127_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_127_2,
        );
        // D s_127_4: read-var Nshadow#322:i
        let s_127_4: i128 = fn_state.Nshadow_322;
        // D s_127_5: call __UNKNOWN_bits(s_127_4)
        let s_127_5: Bits = u__UNKNOWN_bits(state, tracer, s_127_4);
        // D s_127_6: read-var s2fault:struct
        let s_127_6: ProductType1d757adad216cdef = fn_state.s2fault;
        // D s_127_7: create-product struct = ["s_127_6", "s_127_1", "s_127_3", "s_127_5"]
        let s_127_7: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_127_6,
            _1: s_127_1,
            _2: s_127_3,
            _3: s_127_5,
        };
        // D s_127_8: write-var return_value <= s_127_7
        fn_state.return_value = s_127_7;
        // N s_127_9: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_128_0: const #() : ()
        let s_128_0: () = ();
        // S s_128_1: call EL2Enabled(s_128_0)
        let s_128_1: bool = EL2Enabled(state, tracer, s_128_0);
        // D s_128_2: write-var gs#19669 <= s_128_1
        fn_state.gs_19669 = s_128_1;
        // N s_128_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_129_0: const #7u : u32
        let s_129_0: u32 = 7;
        // D s_129_1: write-var fault.16 <= s_129_0
        fn_state.fault._16 = s_129_0;
        // C s_129_2: const #0s : i
        let s_129_2: i128 = 0;
        // D s_129_3: write-var fault.9 <= s_129_2
        fn_state.fault._9 = s_129_2;
        // C s_129_4: const #() : ()
        let s_129_4: () = ();
        // S s_129_5: call __UNKNOWN_AddressDescriptor(s_129_4)
        let s_129_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_129_4,
        );
        // C s_129_6: const #() : ()
        let s_129_6: () = ();
        // S s_129_7: call __UNKNOWN_TTWState(s_129_6)
        let s_129_7: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_129_6,
        );
        // D s_129_8: read-var Nshadow#322:i
        let s_129_8: i128 = fn_state.Nshadow_322;
        // D s_129_9: call __UNKNOWN_bits(s_129_8)
        let s_129_9: Bits = u__UNKNOWN_bits(state, tracer, s_129_8);
        // D s_129_10: read-var fault:struct
        let s_129_10: ProductType1d757adad216cdef = fn_state.fault;
        // D s_129_11: create-product struct = ["s_129_10", "s_129_5", "s_129_7", "s_129_9"]
        let s_129_11: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_129_10,
            _1: s_129_5,
            _2: s_129_7,
            _3: s_129_9,
        };
        // D s_129_12: write-var return_value <= s_129_11
        fn_state.return_value = s_129_11;
        // N s_129_13: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_130_0: read-var walkstate.7:struct
        let s_130_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_130_1: write-var ga#14983 <= s_130_0
        fn_state.ga_14983 = s_130_0;
        // D s_130_2: read-var ga#14983.5:struct
        let s_130_2: u32 = fn_state.ga_14983._5;
        // D s_130_3: write-var walkaddress.2.5 <= s_130_2
        fn_state.walkaddress._2._5 = s_130_2;
        // N s_130_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_131_0: const #21232u : u32
        let s_131_0: u32 = 21232;
        // D s_131_1: read-reg s_131_0:u8
        let s_131_1: bool = {
            let value = state.read_register::<bool>(s_131_0 as isize);
            tracer.read_register(s_131_0 as isize, value);
            value
        };
        // D s_131_2: not s_131_1
        let s_131_2: bool = !s_131_1;
        // D s_131_3: write-var gs#19660 <= s_131_2
        fn_state.gs_19660 = s_131_2;
        // N s_131_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_132_0: const #102552u : u32
        let s_132_0: u32 = 102552;
        // D s_132_1: read-reg s_132_0:struct
        let s_132_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // D s_132_2: call _get_HCR_EL2_Type_VM(s_132_1)
        let s_132_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_132_1);
        // D s_132_3: cast zx s_132_2 -> bv
        let s_132_3: Bits = Bits::new(s_132_2 as u128, 1u16);
        // C s_132_4: const #1u : u8
        let s_132_4: bool = true;
        // C s_132_5: cast zx s_132_4 -> bv
        let s_132_5: Bits = Bits::new(s_132_4 as u128, 1u16);
        // D s_132_6: cmp-eq s_132_3 s_132_5
        let s_132_6: bool = ((s_132_3) == (s_132_5));
        // D s_132_7: write-var gs#19659 <= s_132_6
        fn_state.gs_19659 = s_132_6;
        // N s_132_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call EL2Enabled(s_133_0)
        let s_133_1: bool = EL2Enabled(state, tracer, s_133_0);
        // D s_133_2: write-var gs#19658 <= s_133_1
        fn_state.gs_19658 = s_133_1;
        // N s_133_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_134_0: const #() : ()
        let s_134_0: () = ();
        // S s_134_1: call NormalNCMemAttr(s_134_0)
        let s_134_1: ProductTypef170cab34335b70c = NormalNCMemAttr(
            state,
            tracer,
            s_134_0,
        );
        // D s_134_2: write-var walkaddress.2 <= s_134_1
        fn_state.walkaddress._2 = s_134_1;
        // D s_134_3: read-var walkstate.7:struct
        let s_134_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_134_4: write-var ga#14977 <= s_134_3
        fn_state.ga_14977 = s_134_3;
        // D s_134_5: read-var ga#14977.7:struct
        let s_134_5: bool = fn_state.ga_14977._7;
        // D s_134_6: write-var walkaddress.2.7 <= s_134_5
        fn_state.walkaddress._2._7 = s_134_5;
        // N s_134_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_135_0: const #6u : u32
        let s_135_0: u32 = 6;
        // D s_135_1: write-var fault.16 <= s_135_0
        fn_state.fault._16 = s_135_0;
        // C s_135_2: const #0s : i
        let s_135_2: i128 = 0;
        // D s_135_3: write-var fault.9 <= s_135_2
        fn_state.fault._9 = s_135_2;
        // C s_135_4: const #() : ()
        let s_135_4: () = ();
        // S s_135_5: call __UNKNOWN_AddressDescriptor(s_135_4)
        let s_135_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_135_4,
        );
        // C s_135_6: const #() : ()
        let s_135_6: () = ();
        // S s_135_7: call __UNKNOWN_TTWState(s_135_6)
        let s_135_7: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_135_6,
        );
        // D s_135_8: read-var Nshadow#322:i
        let s_135_8: i128 = fn_state.Nshadow_322;
        // D s_135_9: call __UNKNOWN_bits(s_135_8)
        let s_135_9: Bits = u__UNKNOWN_bits(state, tracer, s_135_8);
        // D s_135_10: read-var fault:struct
        let s_135_10: ProductType1d757adad216cdef = fn_state.fault;
        // D s_135_11: create-product struct = ["s_135_10", "s_135_5", "s_135_7", "s_135_9"]
        let s_135_11: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_135_10,
            _1: s_135_5,
            _2: s_135_7,
            _3: s_135_9,
        };
        // D s_135_12: write-var return_value <= s_135_11
        fn_state.return_value = s_135_11;
        // N s_135_13: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_136_0: read-var regime:u32
        let s_136_0: u32 = fn_state.regime;
        // D s_136_1: read-var va:u64
        let s_136_1: u64 = fn_state.va;
        // D s_136_2: call AArch64_S1EPD(s_136_0, s_136_1)
        let s_136_2: bool = AArch64_S1EPD(state, tracer, s_136_0, s_136_1);
        // D s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // C s_136_4: const #1u : u8
        let s_136_4: bool = true;
        // C s_136_5: cast zx s_136_4 -> bv
        let s_136_5: Bits = Bits::new(s_136_4 as u128, 1u16);
        // D s_136_6: cmp-eq s_136_3 s_136_5
        let s_136_6: bool = ((s_136_3) == (s_136_5));
        // D s_136_7: write-var gs#19624 <= s_136_6
        fn_state.gs_19624 = s_136_6;
        // N s_136_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_137_0: read-var accdesc.25:struct
        let s_137_0: u32 = fn_state.accdesc._25;
        // D s_137_1: read-var walkparams.36:struct
        let s_137_1: u32 = fn_state.walkparams._36;
        // D s_137_2: read-var regime:u32
        let s_137_2: u32 = fn_state.regime;
        // D s_137_3: read-var va:u64
        let s_137_3: u64 = fn_state.va;
        // D s_137_4: call AArch64_GetS1TLBContext(s_137_2, s_137_0, s_137_3, s_137_1)
        let s_137_4: ProductTypec0d0fb0603850c4c = AArch64_GetS1TLBContext(
            state,
            tracer,
            s_137_2,
            s_137_0,
            s_137_3,
            s_137_1,
        );
        // D s_137_5: write-var tlbcontext <= s_137_4
        fn_state.tlbcontext = s_137_4;
        // D s_137_6: read-var tlbcontext:struct
        let s_137_6: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_137_7: call S1TLBLookup(s_137_6)
        let s_137_7: ProductTypeeb828c17bbe5e68 = S1TLBLookup(state, tracer, s_137_6);
        // D s_137_8: write-var tlbentry <= s_137_7
        fn_state.tlbentry = s_137_7;
        // D s_137_9: read-var tlbentry.0:struct
        let s_137_9: ProductTypee47dd77b186df56e = fn_state.tlbentry._0;
        // D s_137_10: write-var tlbrecord <= s_137_9
        fn_state.tlbrecord = s_137_9;
        // D s_137_11: read-var walkparams.3:struct
        let s_137_11: bool = fn_state.walkparams._3;
        // D s_137_12: read-var walkparams.36:struct
        let s_137_12: u32 = fn_state.walkparams._36;
        // D s_137_13: read-var tlbrecord.5:struct
        let s_137_13: ProductType96e7acababe246a1 = fn_state.tlbrecord._5;
        // D s_137_14: read-var va:u64
        let s_137_14: u64 = fn_state.va;
        // D s_137_15: call StageOA(s_137_14, s_137_11, s_137_12, s_137_13)
        let s_137_15: ProductTypeda0231e9dc169f81 = StageOA(
            state,
            tracer,
            s_137_14,
            s_137_11,
            s_137_12,
            s_137_13,
        );
        // D s_137_16: write-var oa <= s_137_15
        fn_state.oa = s_137_15;
        // D s_137_17: read-var tlbentry.1:struct
        let s_137_17: bool = fn_state.tlbentry._1;
        // N s_137_18: branch s_137_17 b169 b138
        if s_137_17 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#19633 <= s_138_0
        fn_state.gs_19633 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_139_0: read-var gs#19633:u8
        let s_139_0: bool = fn_state.gs_19633;
        // N s_139_1: branch s_139_0 b147 b140
        if s_139_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#19645 <= s_140_0
        fn_state.gs_19645 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_141_0: read-var gs#19645:u8
        let s_141_0: bool = fn_state.gs_19645;
        // N s_141_1: branch s_141_0 b143 b142
        if s_141_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_142_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_143_0: read-var tlbrecord.5:struct
        let s_143_0: ProductType96e7acababe246a1 = fn_state.tlbrecord._5;
        // D s_143_1: write-var ga#14948 <= s_143_0
        fn_state.ga_14948 = s_143_0;
        // D s_143_2: read-var ga#14948.6:struct
        let s_143_2: i128 = fn_state.ga_14948._6;
        // D s_143_3: write-var fault.9 <= s_143_2
        fn_state.fault._9 = s_143_2;
        // D s_143_4: read-var Nshadow#322:i
        let s_143_4: i128 = fn_state.Nshadow_322;
        // D s_143_5: call __id(s_143_4)
        let s_143_5: i128 = u__id(state, tracer, s_143_4);
        // C s_143_6: const #1s : i
        let s_143_6: i128 = 1;
        // D s_143_7: sub s_143_5 s_143_6
        let s_143_7: i128 = ((s_143_5) - (s_143_6));
        // C s_143_8: const #0s : i
        let s_143_8: i128 = 0;
        // D s_143_9: cmp-le s_143_8 s_143_7
        let s_143_9: bool = ((s_143_8) <= (s_143_7));
        // N s_143_10: branch s_143_9 b146 b144
        if s_143_9 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var gs#19651 <= s_144_0
        fn_state.gs_19651 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_145_0: read-var gs#19651:u8
        let s_145_0: bool = fn_state.gs_19651;
        // N s_145_1: assert s_145_0
        let s_145_1: () = assert!(s_145_0);
        // C s_145_2: const #() : ()
        let s_145_2: () = ();
        // S s_145_3: call __UNKNOWN_AddressDescriptor(s_145_2)
        let s_145_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_145_2,
        );
        // D s_145_4: read-var tlbrecord.5:struct
        let s_145_4: ProductType96e7acababe246a1 = fn_state.tlbrecord._5;
        // D s_145_5: read-var tlbrecord.3:struct
        let s_145_5: u128 = fn_state.tlbrecord._3;
        // C s_145_6: const #1s : i
        let s_145_6: i128 = 1;
        // D s_145_7: read-var Nshadow#322:i
        let s_145_7: i128 = fn_state.Nshadow_322;
        // D s_145_8: sub s_145_7 s_145_6
        let s_145_8: i128 = ((s_145_7) - (s_145_6));
        // D s_145_9: cast reint s_145_8 -> i64
        let s_145_9: i64 = (s_145_8 as i64);
        // C s_145_10: const #0s : i
        let s_145_10: i128 = 0;
        // D s_145_11: cast zx s_145_5 -> bv
        let s_145_11: Bits = Bits::new(s_145_5 as u128, 128u16);
        // D s_145_12: cast zx s_145_9 -> i
        let s_145_12: i128 = (i128::try_from(s_145_9).unwrap());
        // C s_145_13: const #1s : i64
        let s_145_13: i64 = 1;
        // C s_145_14: cast zx s_145_13 -> i
        let s_145_14: i128 = (i128::try_from(s_145_13).unwrap());
        // D s_145_15: sub s_145_12 s_145_10
        let s_145_15: i128 = ((s_145_12) - (s_145_10));
        // D s_145_16: add s_145_15 s_145_14
        let s_145_16: i128 = (s_145_15 + s_145_14);
        // D s_145_17: bit-extract s_145_11 s_145_10 s_145_16
        let s_145_17: Bits = (Bits::new(
            ((s_145_11) >> (s_145_10)).value(),
            u16::try_from(s_145_16).unwrap(),
        ));
        // D s_145_18: read-var fault:struct
        let s_145_18: ProductType1d757adad216cdef = fn_state.fault;
        // D s_145_19: create-product struct = ["s_145_18", "s_145_3", "s_145_4", "s_145_17"]
        let s_145_19: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_145_18,
            _1: s_145_3,
            _2: s_145_4,
            _3: s_145_17,
        };
        // D s_145_20: write-var return_value <= s_145_19
        fn_state.return_value = s_145_19;
        // N s_145_21: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_146_0: read-var Nshadow#322:i
        let s_146_0: i128 = fn_state.Nshadow_322;
        // D s_146_1: call __id(s_146_0)
        let s_146_1: i128 = u__id(state, tracer, s_146_0);
        // C s_146_2: const #1s : i
        let s_146_2: i128 = 1;
        // D s_146_3: sub s_146_1 s_146_2
        let s_146_3: i128 = ((s_146_1) - (s_146_2));
        // C s_146_4: const #128s : i
        let s_146_4: i128 = 128;
        // D s_146_5: cmp-lt s_146_3 s_146_4
        let s_146_5: bool = ((s_146_3) < (s_146_4));
        // D s_146_6: write-var gs#19651 <= s_146_5
        fn_state.gs_19651 = s_146_5;
        // N s_146_7: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_147_0: read-var walkparams.12:struct
        let s_147_0: bool = fn_state.walkparams._12;
        // D s_147_1: read-var walkparams.14:struct
        let s_147_1: bool = fn_state.walkparams._14;
        // D s_147_2: cast zx s_147_0 -> bv
        let s_147_2: Bits = Bits::new(s_147_0 as u128, 1u16);
        // D s_147_3: cast zx s_147_1 -> bv
        let s_147_3: Bits = Bits::new(s_147_1 as u128, 1u16);
        // D s_147_4: cast reint s_147_2 -> u128
        let s_147_4: u128 = (s_147_2.value() as u128);
        // D s_147_5: size-of s_147_2
        let s_147_5: u16 = s_147_2.length();
        // D s_147_6: cast reint s_147_3 -> u128
        let s_147_6: u128 = (s_147_3.value() as u128);
        // D s_147_7: size-of s_147_3
        let s_147_7: u16 = s_147_3.length();
        // D s_147_8: lsl s_147_4 s_147_7
        let s_147_8: u128 = s_147_4 << s_147_7;
        // D s_147_9: or s_147_8 s_147_6
        let s_147_9: u128 = ((s_147_8) | (s_147_6));
        // D s_147_10: add s_147_5 s_147_7
        let s_147_10: u16 = (s_147_5 + s_147_7);
        // D s_147_11: create-bits s_147_9 s_147_10
        let s_147_11: Bits = Bits::new(s_147_9, s_147_10);
        // D s_147_12: cast reint s_147_11 -> u8
        let s_147_12: u8 = (s_147_11.value() as u8);
        // D s_147_13: cast zx s_147_12 -> bv
        let s_147_13: Bits = Bits::new(s_147_12 as u128, 2u16);
        // C s_147_14: const #3u : u8
        let s_147_14: u8 = 3;
        // C s_147_15: cast zx s_147_14 -> bv
        let s_147_15: Bits = Bits::new(s_147_14 as u128, 2u16);
        // D s_147_16: cmp-eq s_147_13 s_147_15
        let s_147_16: bool = ((s_147_13) == (s_147_15));
        // N s_147_17: branch s_147_16 b165 b148
        if s_147_16 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#19637 <= s_148_0
        fn_state.gs_19637 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_149_0: read-var gs#19637:u8
        let s_149_0: bool = fn_state.gs_19637;
        // N s_149_1: branch s_149_0 b164 b150
        if s_149_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_150_0: const #0u : u8
        let s_150_0: bool = false;
        // D s_150_1: write-var gs#19640 <= s_150_0
        fn_state.gs_19640 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_151_0: read-var gs#19640:u8
        let s_151_0: bool = fn_state.gs_19640;
        // N s_151_1: branch s_151_0 b163 b152
        if s_151_0 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_152_0: const #0u : u8
        let s_152_0: bool = false;
        // D s_152_1: write-var gs#19641 <= s_152_0
        fn_state.gs_19641 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_153_0: read-var gs#19641:u8
        let s_153_0: bool = fn_state.gs_19641;
        // N s_153_1: branch s_153_0 b156 b154
        if s_153_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#19644 <= s_154_0
        fn_state.gs_19644 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_155_0: read-var gs#19644:u8
        let s_155_0: bool = fn_state.gs_19644;
        // D s_155_1: not s_155_0
        let s_155_1: bool = !s_155_0;
        // D s_155_2: write-var gs#19645 <= s_155_1
        fn_state.gs_19645 = s_155_1;
        // N s_155_3: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_156_0: read-var accdesc.1:struct
        let s_156_0: u32 = fn_state.accdesc._1;
        // C s_156_1: const #8u : u32
        let s_156_1: u32 = 8;
        // D s_156_2: cmp-eq s_156_0 s_156_1
        let s_156_2: bool = ((s_156_0) == (s_156_1));
        // N s_156_3: branch s_156_2 b162 b157
        if s_156_2 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_157_0: read-var accdesc.1:struct
        let s_157_0: u32 = fn_state.accdesc._1;
        // C s_157_1: const #5u : u32
        let s_157_1: u32 = 5;
        // D s_157_2: cmp-eq s_157_0 s_157_1
        let s_157_2: bool = ((s_157_0) == (s_157_1));
        // N s_157_3: branch s_157_2 b161 b158
        if s_157_2 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_158_0: read-var accdesc.1:struct
        let s_158_0: u32 = fn_state.accdesc._1;
        // C s_158_1: const #6u : u32
        let s_158_1: u32 = 6;
        // D s_158_2: cmp-eq s_158_0 s_158_1
        let s_158_2: bool = ((s_158_0) == (s_158_1));
        // D s_158_3: write-var gs#19642 <= s_158_2
        fn_state.gs_19642 = s_158_2;
        // N s_158_4: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_159_0: read-var gs#19642:u8
        let s_159_0: bool = fn_state.gs_19642;
        // D s_159_1: write-var gs#19643 <= s_159_0
        fn_state.gs_19643 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_160_0: read-var gs#19643:u8
        let s_160_0: bool = fn_state.gs_19643;
        // D s_160_1: not s_160_0
        let s_160_1: bool = !s_160_0;
        // D s_160_2: write-var gs#19644 <= s_160_1
        fn_state.gs_19644 = s_160_1;
        // N s_160_3: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_161_0: const #1u : u8
        let s_161_0: bool = true;
        // D s_161_1: write-var gs#19642 <= s_161_0
        fn_state.gs_19642 = s_161_0;
        // N s_161_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_162_0: const #1u : u8
        let s_162_0: bool = true;
        // D s_162_1: write-var gs#19643 <= s_162_0
        fn_state.gs_19643 = s_162_0;
        // N s_162_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_163_0: read-var accdesc.32:struct
        let s_163_0: bool = fn_state.accdesc._32;
        // D s_163_1: write-var gs#19641 <= s_163_0
        fn_state.gs_19641 = s_163_0;
        // N s_163_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_164_0: read-var tlbrecord.3:struct
        let s_164_0: u128 = fn_state.tlbrecord._3;
        // C s_164_1: const #7s : i
        let s_164_1: i128 = 7;
        // D s_164_2: cast zx s_164_0 -> bv
        let s_164_2: Bits = Bits::new(s_164_0 as u128, 128u16);
        // C s_164_3: const #1u : u64
        let s_164_3: u64 = 1;
        // D s_164_4: bit-extract s_164_2 s_164_1 s_164_3
        let s_164_4: Bits = (Bits::new(
            ((s_164_2) >> (s_164_1)).value(),
            u16::try_from(s_164_3).unwrap(),
        ));
        // D s_164_5: cast reint s_164_4 -> u8
        let s_164_5: bool = ((s_164_4.value()) != 0);
        // C s_164_6: const #0s : i
        let s_164_6: i128 = 0;
        // C s_164_7: const #0u : u64
        let s_164_7: u64 = 0;
        // D s_164_8: cast zx s_164_5 -> u64
        let s_164_8: u64 = (s_164_5 as u64);
        // C s_164_9: const #1u : u64
        let s_164_9: u64 = 1;
        // D s_164_10: and s_164_8 s_164_9
        let s_164_10: u64 = ((s_164_8) & (s_164_9));
        // D s_164_11: cmp-eq s_164_10 s_164_9
        let s_164_11: bool = ((s_164_10) == (s_164_9));
        // D s_164_12: lsl s_164_8 s_164_6
        let s_164_12: u64 = s_164_8 << s_164_6;
        // D s_164_13: or s_164_7 s_164_12
        let s_164_13: u64 = ((s_164_7) | (s_164_12));
        // D s_164_14: cmpl s_164_12
        let s_164_14: u64 = !s_164_12;
        // D s_164_15: and s_164_7 s_164_14
        let s_164_15: u64 = ((s_164_7) & (s_164_14));
        // D s_164_16: select s_164_11 s_164_13 s_164_15
        let s_164_16: u64 = if s_164_11 { s_164_13 } else { s_164_15 };
        // D s_164_17: cast trunc s_164_16 -> u8
        let s_164_17: bool = ((s_164_16) != 0);
        // D s_164_18: cast zx s_164_17 -> bv
        let s_164_18: Bits = Bits::new(s_164_17 as u128, 1u16);
        // C s_164_19: const #1u : u8
        let s_164_19: bool = true;
        // C s_164_20: cast zx s_164_19 -> bv
        let s_164_20: Bits = Bits::new(s_164_19 as u128, 1u16);
        // D s_164_21: cmp-eq s_164_18 s_164_20
        let s_164_21: bool = ((s_164_18) == (s_164_20));
        // D s_164_22: write-var gs#19640 <= s_164_21
        fn_state.gs_19640 = s_164_21;
        // N s_164_23: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_165_0: read-var walkparams.24:struct
        let s_165_0: bool = fn_state.walkparams._24;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #1u : u8
        let s_165_2: bool = true;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // N s_165_5: branch s_165_4 b168 b166
        if s_165_4 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_166_0: read-var tlbrecord.3:struct
        let s_166_0: u128 = fn_state.tlbrecord._3;
        // C s_166_1: const #51s : i
        let s_166_1: i128 = 51;
        // D s_166_2: cast zx s_166_0 -> bv
        let s_166_2: Bits = Bits::new(s_166_0 as u128, 128u16);
        // C s_166_3: const #1u : u64
        let s_166_3: u64 = 1;
        // D s_166_4: bit-extract s_166_2 s_166_1 s_166_3
        let s_166_4: Bits = (Bits::new(
            ((s_166_2) >> (s_166_1)).value(),
            u16::try_from(s_166_3).unwrap(),
        ));
        // D s_166_5: cast reint s_166_4 -> u8
        let s_166_5: bool = ((s_166_4.value()) != 0);
        // C s_166_6: const #0s : i
        let s_166_6: i128 = 0;
        // C s_166_7: const #0u : u64
        let s_166_7: u64 = 0;
        // D s_166_8: cast zx s_166_5 -> u64
        let s_166_8: u64 = (s_166_5 as u64);
        // C s_166_9: const #1u : u64
        let s_166_9: u64 = 1;
        // D s_166_10: and s_166_8 s_166_9
        let s_166_10: u64 = ((s_166_8) & (s_166_9));
        // D s_166_11: cmp-eq s_166_10 s_166_9
        let s_166_11: bool = ((s_166_10) == (s_166_9));
        // D s_166_12: lsl s_166_8 s_166_6
        let s_166_12: u64 = s_166_8 << s_166_6;
        // D s_166_13: or s_166_7 s_166_12
        let s_166_13: u64 = ((s_166_7) | (s_166_12));
        // D s_166_14: cmpl s_166_12
        let s_166_14: u64 = !s_166_12;
        // D s_166_15: and s_166_7 s_166_14
        let s_166_15: u64 = ((s_166_7) & (s_166_14));
        // D s_166_16: select s_166_11 s_166_13 s_166_15
        let s_166_16: u64 = if s_166_11 { s_166_13 } else { s_166_15 };
        // D s_166_17: cast trunc s_166_16 -> u8
        let s_166_17: bool = ((s_166_16) != 0);
        // D s_166_18: cast zx s_166_17 -> bv
        let s_166_18: Bits = Bits::new(s_166_17 as u128, 1u16);
        // C s_166_19: const #1u : u8
        let s_166_19: bool = true;
        // C s_166_20: cast zx s_166_19 -> bv
        let s_166_20: Bits = Bits::new(s_166_19 as u128, 1u16);
        // D s_166_21: cmp-eq s_166_18 s_166_20
        let s_166_21: bool = ((s_166_18) == (s_166_20));
        // D s_166_22: write-var gs#19636 <= s_166_21
        fn_state.gs_19636 = s_166_21;
        // N s_166_23: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_167_0: read-var gs#19636:u8
        let s_167_0: bool = fn_state.gs_19636;
        // D s_167_1: write-var gs#19637 <= s_167_0
        fn_state.gs_19637 = s_167_0;
        // N s_167_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_168_0: const #1u : u8
        let s_168_0: bool = true;
        // D s_168_1: write-var gs#19636 <= s_168_0
        fn_state.gs_19636 = s_168_0;
        // N s_168_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_169_0: read-var oa.0:struct
        let s_169_0: u64 = fn_state.oa._0;
        // D s_169_1: read-var walkparams.3:struct
        let s_169_1: bool = fn_state.walkparams._3;
        // D s_169_2: read-var walkparams.28:struct
        let s_169_2: u8 = fn_state.walkparams._28;
        // D s_169_3: read-var walkparams.36:struct
        let s_169_3: u32 = fn_state.walkparams._36;
        // D s_169_4: call AArch64_OAOutOfRange(s_169_0, s_169_1, s_169_2, s_169_3)
        let s_169_4: bool = AArch64_OAOutOfRange(
            state,
            tracer,
            s_169_0,
            s_169_1,
            s_169_2,
            s_169_3,
        );
        // D s_169_5: not s_169_4
        let s_169_5: bool = !s_169_4;
        // D s_169_6: write-var gs#19633 <= s_169_5
        fn_state.gs_19633 = s_169_5;
        // N s_169_7: jump b139
        return block_139(state, tracer, fn_state);
    }
}
