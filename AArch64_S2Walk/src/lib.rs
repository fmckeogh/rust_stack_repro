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
use AArch64_S2ApplyFWBMemAttrs::*;
use AArch64_TTWalkMECID::*;
use VMID_read::*;
use AArch64_OAOutOfRange::*;
use CreateAccDescS2TTW::*;
use u__UNKNOWN_bits::*;
use AArch64_S2NextWalkStateTable::*;
use AArch64_S2SLTTEntryAddress::*;
use NormalNCMemAttr::*;
use S2TLBLookup::*;
use CreateAccDescTTEUpdate::*;
use TranslationSize::*;
use AArch64_ContiguousBitFaults::*;
use AArch64_MemSwapTableDesc::*;
use FetchDescriptor::*;
use StageOA::*;
use u__UNKNOWN_TTWState::*;
use AArch64_SS2InitialTTWState::*;
use u_get_HCR_EL2_Type_CD::*;
use AArch64_BlocknTFaults::*;
use EffectiveShareability::*;
use AArch64_TTEntryAddress::*;
use AArch64_S2NextWalkStateLeaf::*;
use Unreachable::*;
use ContiguousSize::*;
use u__id::*;
use u__UNKNOWN_AddressDescriptor::*;
use AArch64_DecodeDescriptorType::*;
use AArch64_GetS2TLBContext::*;
use AArch64_S2InitialTTWState::*;
use S2TLBCache::*;
use common::*;
pub fn AArch64_S2Walk<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    ipa: ProductTypece7c66ccb2cab13e,
    walkparams: ProductTypeb05ce25a107f0c5e,
    accdesc: ProductType9878976b5bcce9c9,
    s1level: SumTypebf36e919d71ba1d6,
    N: i64,
) -> ProductType4b99944cd5e0b59d {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        Nshadow_321: i64,
        gs_19357: bool,
        ga_14732: ProductTypeda0231e9dc169f81,
        gs_19350: bool,
        gs_19351: bool,
        gs_19356: bool,
        ga_14709: ProductTypef170cab34335b70c,
        descaccess: ProductType9878976b5bcce9c9,
        skl: u8,
        new_descriptor: Bits,
        translation_info: ProductTypeb525737120e184b3,
        gs_19354: bool,
        ga_14572: ProductType96e7acababe246a1,
        walkaddress: ProductTypece7c66ccb2cab13e,
        gs_19468: bool,
        gs_19347: bool,
        tlbrecord: ProductTypee47dd77b186df56e,
        descaddress: ProductTypeda0231e9dc169f81,
        gs_19358: bool,
        gs_19471: bool,
        ga_14676: ProductType963c597a88a9ddbc,
        ga_14606: ProductTypeb4cea7287e2eb9d6,
        ga_14663: ProductTypeb4cea7287e2eb9d6,
        gs_19436: bool,
        gs_19440: bool,
        ipa_64: u64,
        gs_19472: bool,
        ga_14592: ProductTypef170cab34335b70c,
        desctype: u32,
        gs_19355: bool,
        gs_19467: bool,
        descriptor: Bits,
        walkaccess: ProductType9878976b5bcce9c9,
        fault: ProductType1d757adad216cdef,
        gs_19392: bool,
        oa: ProductTypeda0231e9dc169f81,
        return_value: ProductType4b99944cd5e0b59d,
        ga_14735: ProductTypeda0231e9dc169f81,
        gs_19391: bool,
        gs_19359: bool,
        gs_446427: ProductType1f0c48777d4d25a0,
        gs_19415: bool,
        gs_19437: bool,
        walkstate: ProductType96e7acababe246a1,
        ga_14584: ProductTypeda0231e9dc169f81,
        u_884: ProductTypeda0231e9dc169f81,
        tlbentry: ProductTypeeb828c17bbe5e68,
        fault_in: ProductType1d757adad216cdef,
        ipa: ProductTypece7c66ccb2cab13e,
        walkparams: ProductTypeb05ce25a107f0c5e,
        accdesc: ProductType9878976b5bcce9c9,
        s1level: SumTypebf36e919d71ba1d6,
        N: i64,
    }
    let fn_state = FunctionState {
        fault_in,
        ipa,
        walkparams,
        accdesc,
        s1level,
        N,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_0_0: read-var N:i64
        let s_0_0: i64 = fn_state.N;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var Nshadow#321 <= s_0_2
        fn_state.Nshadow_321 = s_0_2;
        // D s_0_4: read-var fault_in:struct
        let s_0_4: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_5: write-var fault <= s_0_4
        fn_state.fault = s_0_4;
        // D s_0_6: read-var ipa.3:struct
        let s_0_6: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_0_7: write-var ga#14735 <= s_0_6
        fn_state.ga_14735 = s_0_6;
        // D s_0_8: read-var ga#14735.0:struct
        let s_0_8: u64 = fn_state.ga_14735._0;
        // C s_0_9: const #64s : i
        let s_0_9: i128 = 64;
        // D s_0_10: cast zx s_0_8 -> bv
        let s_0_10: Bits = Bits::new(s_0_8 as u128, 56u16);
        // D s_0_11: bits-cast zx s_0_10 -> bv length s_0_9
        let s_0_11: Bits = s_0_10.zero_extend(s_0_9);
        // D s_0_12: cast reint s_0_11 -> u64
        let s_0_12: u64 = (s_0_11.value() as u64);
        // D s_0_13: write-var ipa_64 <= s_0_12
        fn_state.ipa_64 = s_0_12;
        // C s_0_14: const #19088u : u32
        let s_0_14: u32 = 19088;
        // D s_0_15: read-reg s_0_14:u8
        let s_0_15: bool = {
            let value = state.read_register::<bool>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // N s_0_16: branch s_0_15 b88 b1
        if s_0_15 {
            return block_88(state, tracer, fn_state);
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
        // D s_2_0: read-var accdesc.25:struct
        let s_2_0: u32 = fn_state.accdesc._25;
        // C s_2_1: const #3u : u32
        let s_2_1: u32 = 3;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b87 b3
        if s_2_2 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_3_0: read-var accdesc.25:struct
        let s_3_0: u32 = fn_state.accdesc._25;
        // D s_3_1: read-var walkparams:struct
        let s_3_1: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_3_2: call AArch64_S2InitialTTWState(s_3_0, s_3_1)
        let s_3_2: ProductType96e7acababe246a1 = AArch64_S2InitialTTWState(
            state,
            tracer,
            s_3_0,
            s_3_1,
        );
        // D s_3_3: write-var walkstate <= s_3_2
        fn_state.walkstate = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_4_0: read-var accdesc:struct
        let s_4_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_1: call CreateAccDescS2TTW(s_4_0)
        let s_4_1: ProductType9878976b5bcce9c9 = CreateAccDescS2TTW(
            state,
            tracer,
            s_4_0,
        );
        // D s_4_2: write-var walkaccess <= s_4_1
        fn_state.walkaccess = s_4_1;
        // C s_4_3: const #0u : u8
        let s_4_3: u8 = 0;
        // D s_4_4: write-var skl <= s_4_3
        fn_state.skl = s_4_3;
        // D s_4_5: read-var ipa.7:struct
        let s_4_5: u64 = fn_state.ipa._7;
        // D s_4_6: write-var walkaddress.7 <= s_4_5
        fn_state.walkaddress._7 = s_4_5;
        // D s_4_7: read-var walkparams.5:struct
        let s_4_7: bool = fn_state.walkparams._5;
        // D s_4_8: read-var accdesc.25:struct
        let s_4_8: u32 = fn_state.accdesc._25;
        // C s_4_9: const #4u : u32
        let s_4_9: u32 = 4;
        // D s_4_10: call AArch64_TTWalkMECID(s_4_7, s_4_9, s_4_8)
        let s_4_10: u16 = AArch64_TTWalkMECID(state, tracer, s_4_7, s_4_9, s_4_8);
        // D s_4_11: write-var walkaddress.1 <= s_4_10
        fn_state.walkaddress._1 = s_4_10;
        // C s_4_12: const #102552u : u32
        let s_4_12: u32 = 102552;
        // D s_4_13: read-reg s_4_12:struct
        let s_4_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_12 as isize);
            tracer.read_register(s_4_12 as isize, value);
            value
        };
        // D s_4_14: call _get_HCR_EL2_Type_CD(s_4_13)
        let s_4_14: bool = u_get_HCR_EL2_Type_CD(state, tracer, s_4_13);
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 1u16);
        // C s_4_16: const #1u : u8
        let s_4_16: bool = true;
        // C s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 1u16);
        // D s_4_18: cmp-eq s_4_15 s_4_17
        let s_4_18: bool = ((s_4_15) == (s_4_17));
        // N s_4_19: branch s_4_18 b86 b5
        if s_4_18 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_5_0: read-var walkstate.7:struct
        let s_5_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_5_1: write-var walkaddress.2 <= s_5_0
        fn_state.walkaddress._2 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_6_0: read-var walkaddress.2:struct
        let s_6_0: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // D s_6_1: call EffectiveShareability(s_6_0)
        let s_6_1: u32 = EffectiveShareability(state, tracer, s_6_0);
        // D s_6_2: write-var walkaddress.2.5 <= s_6_1
        fn_state.walkaddress._2._5 = s_6_1;
        // D s_6_3: read-var tlbcontext:struct
        let s_6_3: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_6_4: write-var walkaddress.6 <= s_6_3
        fn_state.walkaddress._6 = s_6_3;
        // D s_6_5: read-var ipa.3:struct
        let s_6_5: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_6_6: write-var ga#14732 <= s_6_5
        fn_state.ga_14732 = s_6_5;
        // D s_6_7: read-var ga#14732.0:struct
        let s_6_7: u64 = fn_state.ga_14732._0;
        // D s_6_8: read-var walkstate.0:struct
        let s_6_8: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_6_9: read-var walkparams:struct
        let s_6_9: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_6_10: call AArch64_S2SLTTEntryAddress(s_6_9, s_6_7, s_6_8)
        let s_6_10: ProductTypeda0231e9dc169f81 = AArch64_S2SLTTEntryAddress(
            state,
            tracer,
            s_6_9,
            s_6_7,
            s_6_8,
        );
        // D s_6_11: write-var descaddress <= s_6_10
        fn_state.descaddress = s_6_10;
        // D s_6_12: read-var descaddress.0:struct
        let s_6_12: u64 = fn_state.descaddress._0;
        // D s_6_13: read-var walkparams.2:struct
        let s_6_13: bool = fn_state.walkparams._2;
        // D s_6_14: read-var walkparams.14:struct
        let s_6_14: u8 = fn_state.walkparams._14;
        // D s_6_15: read-var walkparams.26:struct
        let s_6_15: u32 = fn_state.walkparams._26;
        // D s_6_16: call AArch64_OAOutOfRange(s_6_12, s_6_13, s_6_14, s_6_15)
        let s_6_16: bool = AArch64_OAOutOfRange(
            state,
            tracer,
            s_6_12,
            s_6_13,
            s_6_14,
            s_6_15,
        );
        // N s_6_17: branch s_6_16 b85 b7
        if s_6_16 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_8_0: read-var walkstate.6:struct
        let s_8_0: i128 = fn_state.walkstate._6;
        // D s_8_1: write-var fault.9 <= s_8_0
        fn_state.fault._9 = s_8_0;
        // D s_8_2: read-var descaddress:struct
        let s_8_2: ProductTypeda0231e9dc169f81 = fn_state.descaddress;
        // D s_8_3: write-var walkaddress.3 <= s_8_2
        fn_state.walkaddress._3 = s_8_2;
        // C s_8_4: const #() : ()
        let s_8_4: () = ();
        // S s_8_5: call VMID_read(s_8_4)
        let s_8_5: u16 = VMID_read(state, tracer, s_8_4);
        // S s_8_6: cast zx s_8_5 -> bv
        let s_8_6: Bits = Bits::new(s_8_5 as u128, 16u16);
        // D s_8_7: create-sum enum = 1:"s_8_6"
        let s_8_7: SumType755586eec3e2b646 = SumType755586eec3e2b646::_1(s_8_6);
        // C s_8_8: const #() : ()
        let s_8_8: () = ();
        // D s_8_9: create-sum enum = 0:"s_8_8"
        let s_8_9: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_8_8);
        // D s_8_10: read-var walkaddress.7:struct
        let s_8_10: u64 = fn_state.walkaddress._7;
        // D s_8_11: read-var walkstate.6:struct
        let s_8_11: i128 = fn_state.walkstate._6;
        // D s_8_12: read-var ipa_64:u64
        let s_8_12: u64 = fn_state.ipa_64;
        // D s_8_13: create-product struct = ["s_8_12", "s_8_11"]
        let s_8_13: ProductType963c597a88a9ddbc = ProductType963c597a88a9ddbc {
            _0: s_8_12,
            _1: s_8_11,
        };
        // D s_8_14: write-var ga#14676 <= s_8_13
        fn_state.ga_14676 = s_8_13;
        // D s_8_15: read-var ga#14676.0:struct
        let s_8_15: u64 = fn_state.ga_14676._0;
        // D s_8_16: cast zx s_8_15 -> bv
        let s_8_16: Bits = Bits::new(s_8_15 as u128, 64u16);
        // D s_8_17: write-var gs#446427.0 <= s_8_16
        fn_state.gs_446427._0 = s_8_16;
        // D s_8_18: read-var ga#14676.1:struct
        let s_8_18: i128 = fn_state.ga_14676._1;
        // D s_8_19: write-var gs#446427.1 <= s_8_18
        fn_state.gs_446427._1 = s_8_18;
        // D s_8_20: read-var gs#446427:struct
        let s_8_20: ProductType1f0c48777d4d25a0 = fn_state.gs_446427;
        // D s_8_21: create-sum enum = 1:"s_8_20"
        let s_8_21: SumType3cca557f9e907281 = SumType3cca557f9e907281::_1(s_8_20);
        // C s_8_22: const #() : ()
        let s_8_22: () = ();
        // D s_8_23: create-sum enum = 0:"s_8_22"
        let s_8_23: SumTypefc0aa8a49e605a17 = SumTypefc0aa8a49e605a17::_0(s_8_22);
        // D s_8_24: read-var walkparams:struct
        let s_8_24: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_8_25: create-sum enum = 1:"s_8_24"
        let s_8_25: SumType3436044442b382d9 = SumType3436044442b382d9::_1(s_8_24);
        // D s_8_26: read-var walkaddress.2:struct
        let s_8_26: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // C s_8_27: const #4u : u32
        let s_8_27: u32 = 4;
        // D s_8_28: read-var s1level:enum
        let s_8_28: SumTypebf36e919d71ba1d6 = fn_state.s1level;
        // D s_8_29: create-product struct = ["s_8_9", "s_8_26", "s_8_27", "s_8_28", "s_8_23", "s_8_21", "s_8_25", "s_8_10", "s_8_7"]
        let s_8_29: ProductTypeb525737120e184b3 = ProductTypeb525737120e184b3 {
            _0: s_8_9,
            _1: s_8_26,
            _2: s_8_27,
            _3: s_8_28,
            _4: s_8_23,
            _5: s_8_21,
            _6: s_8_25,
            _7: s_8_10,
            _8: s_8_7,
        };
        // D s_8_30: write-var translation_info <= s_8_29
        fn_state.translation_info = s_8_29;
        // D s_8_31: read-var walkparams.4:struct
        let s_8_31: bool = fn_state.walkparams._4;
        // D s_8_32: read-var Nshadow#321:i64
        let s_8_32: i64 = fn_state.Nshadow_321;
        // D s_8_33: cast zx s_8_32 -> i
        let s_8_33: i128 = (i128::try_from(s_8_32).unwrap());
        // D s_8_34: cast reint s_8_33 -> i64
        let s_8_34: i64 = (s_8_33 as i64);
        // D s_8_35: read-var walkaddress:struct
        let s_8_35: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_8_36: read-var walkaccess:struct
        let s_8_36: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_8_37: read-var fault:struct
        let s_8_37: ProductType1d757adad216cdef = fn_state.fault;
        // D s_8_38: read-var translation_info:struct
        let s_8_38: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_8_39: call FetchDescriptor(s_8_31, s_8_35, s_8_36, s_8_37, s_8_34, s_8_38)
        let s_8_39: ProductTypeb4cea7287e2eb9d6 = FetchDescriptor(
            state,
            tracer,
            s_8_31,
            s_8_35,
            s_8_36,
            s_8_37,
            s_8_34,
            s_8_38,
        );
        // D s_8_40: write-var ga#14606 <= s_8_39
        fn_state.ga_14606 = s_8_39;
        // D s_8_41: read-var ga#14606.0:struct
        let s_8_41: ProductType1d757adad216cdef = fn_state.ga_14606._0;
        // D s_8_42: read-var ga#14606.1:struct
        let s_8_42: Bits = fn_state.ga_14606._1;
        // D s_8_43: write-var fault <= s_8_41
        fn_state.fault = s_8_41;
        // D s_8_44: write-var descriptor <= s_8_42
        fn_state.descriptor = s_8_42;
        // D s_8_45: read-var fault.16:struct
        let s_8_45: u32 = fn_state.fault._16;
        // C s_8_46: const #0u : u32
        let s_8_46: u32 = 0;
        // D s_8_47: cmp-eq s_8_45 s_8_46
        let s_8_47: bool = ((s_8_45) == (s_8_46));
        // N s_8_48: branch s_8_47 b84 b9
        if s_8_47 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_10_0: read-var descriptor:bv
        let s_10_0: Bits = fn_state.descriptor;
        // D s_10_1: write-var new_descriptor <= s_10_0
        fn_state.new_descriptor = s_10_0;
        // D s_10_2: read-var walkparams.2:struct
        let s_10_2: bool = fn_state.walkparams._2;
        // D s_10_3: read-var walkparams.3:struct
        let s_10_3: bool = fn_state.walkparams._3;
        // D s_10_4: read-var walkparams.26:struct
        let s_10_4: u32 = fn_state.walkparams._26;
        // D s_10_5: read-var walkstate.6:struct
        let s_10_5: i128 = fn_state.walkstate._6;
        // D s_10_6: read-var descriptor:bv
        let s_10_6: Bits = fn_state.descriptor;
        // D s_10_7: call AArch64_DecodeDescriptorType(s_10_6, s_10_2, s_10_3, s_10_4, s_10_5)
        let s_10_7: u32 = AArch64_DecodeDescriptorType(
            state,
            tracer,
            s_10_6,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
        );
        // D s_10_8: write-var desctype <= s_10_7
        fn_state.desctype = s_10_7;
        // C s_10_9: const #0u : u32
        let s_10_9: u32 = 0;
        // D s_10_10: read-var desctype:u32
        let s_10_10: u32 = fn_state.desctype;
        // D s_10_11: cmp-eq s_10_9 s_10_10
        let s_10_11: bool = ((s_10_9) == (s_10_10));
        // D s_10_12: not s_10_11
        let s_10_12: bool = !s_10_11;
        // N s_10_13: branch s_10_12 b79 b11
        if s_10_12 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_11_0: read-var walkstate:struct
        let s_11_0: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_11_1: read-var walkparams:struct
        let s_11_1: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_11_2: read-var descriptor:bv
        let s_11_2: Bits = fn_state.descriptor;
        // D s_11_3: call AArch64_S2NextWalkStateTable(s_11_0, s_11_1, s_11_2)
        let s_11_3: ProductType96e7acababe246a1 = AArch64_S2NextWalkStateTable(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
        );
        // D s_11_4: write-var walkstate <= s_11_3
        fn_state.walkstate = s_11_3;
        // D s_11_5: read-var walkparams.2:struct
        let s_11_5: bool = fn_state.walkparams._2;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 1u16);
        // C s_11_7: const #1u : u8
        let s_11_7: bool = true;
        // C s_11_8: cast zx s_11_7 -> bv
        let s_11_8: Bits = Bits::new(s_11_7 as u128, 1u16);
        // D s_11_9: cmp-eq s_11_6 s_11_8
        let s_11_9: bool = ((s_11_6) == (s_11_8));
        // N s_11_10: branch s_11_9 b78 b12
        if s_11_9 {
            return block_78(state, tracer, fn_state);
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
        let s_12_0: u8 = 0;
        // D s_12_1: write-var skl <= s_12_0
        fn_state.skl = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_13_0: read-var walkstate.6:struct
        let s_13_0: i128 = fn_state.walkstate._6;
        // D s_13_1: read-var walkparams.2:struct
        let s_13_1: bool = fn_state.walkparams._2;
        // D s_13_2: read-var walkparams.26:struct
        let s_13_2: u32 = fn_state.walkparams._26;
        // D s_13_3: read-var walkparams.29:struct
        let s_13_3: u8 = fn_state.walkparams._29;
        // D s_13_4: read-var walkstate.0:struct
        let s_13_4: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_13_5: read-var skl:u8
        let s_13_5: u8 = fn_state.skl;
        // D s_13_6: read-var ipa_64:u64
        let s_13_6: u64 = fn_state.ipa_64;
        // D s_13_7: call AArch64_TTEntryAddress(s_13_0, s_13_1, s_13_5, s_13_2, s_13_3, s_13_6, s_13_4)
        let s_13_7: ProductTypeda0231e9dc169f81 = AArch64_TTEntryAddress(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_5,
            s_13_2,
            s_13_3,
            s_13_6,
            s_13_4,
        );
        // D s_13_8: write-var descaddress <= s_13_7
        fn_state.descaddress = s_13_7;
        // D s_13_9: read-var descaddress.0:struct
        let s_13_9: u64 = fn_state.descaddress._0;
        // D s_13_10: read-var walkparams.2:struct
        let s_13_10: bool = fn_state.walkparams._2;
        // D s_13_11: read-var walkparams.14:struct
        let s_13_11: u8 = fn_state.walkparams._14;
        // D s_13_12: read-var walkparams.26:struct
        let s_13_12: u32 = fn_state.walkparams._26;
        // D s_13_13: call AArch64_OAOutOfRange(s_13_9, s_13_10, s_13_11, s_13_12)
        let s_13_13: bool = AArch64_OAOutOfRange(
            state,
            tracer,
            s_13_9,
            s_13_10,
            s_13_11,
            s_13_12,
        );
        // N s_13_14: branch s_13_13 b77 b14
        if s_13_13 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_14_0: read-var walkparams.8:struct
        let s_14_0: bool = fn_state.walkparams._8;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // N s_14_5: branch s_14_4 b76 b15
        if s_14_4 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_16_0: read-var walkparams.2:struct
        let s_16_0: bool = fn_state.walkparams._2;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // N s_16_5: branch s_16_4 b75 b17
        if s_16_4 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#19391 <= s_17_0
        fn_state.gs_19391 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_18_0: read-var gs#19391:u8
        let s_18_0: bool = fn_state.gs_19391;
        // N s_18_1: branch s_18_0 b74 b19
        if s_18_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#19392 <= s_19_0
        fn_state.gs_19392 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_20_0: read-var gs#19392:u8
        let s_20_0: bool = fn_state.gs_19392;
        // N s_20_1: branch s_20_0 b73 b21
        if s_20_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_22_0: read-var new_descriptor:bv
        let s_22_0: Bits = fn_state.new_descriptor;
        // D s_22_1: read-var descriptor:bv
        let s_22_1: Bits = fn_state.descriptor;
        // D s_22_2: cmp-ne s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) != (s_22_1));
        // N s_22_3: branch s_22_2 b67 b23
        if s_22_2 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_24_0: read-var new_descriptor:bv
        let s_24_0: Bits = fn_state.new_descriptor;
        // D s_24_1: read-var descriptor:bv
        let s_24_1: Bits = fn_state.descriptor;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // N s_24_3: branch s_24_2 b25 b10
        if s_24_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_25_0: read-var desctype:u32
        let s_25_0: u32 = fn_state.desctype;
        // C s_25_1: const #1u : u32
        let s_25_1: u32 = 1;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // N s_25_3: branch s_25_2 b26 b8
        if s_25_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_26_0: read-var walkparams.2:struct
        let s_26_0: bool = fn_state.walkparams._2;
        // D s_26_1: read-var walkparams.26:struct
        let s_26_1: u32 = fn_state.walkparams._26;
        // D s_26_2: read-var ipa_64:u64
        let s_26_2: u64 = fn_state.ipa_64;
        // D s_26_3: read-var walkstate:struct
        let s_26_3: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_26_4: call StageOA(s_26_2, s_26_0, s_26_1, s_26_3)
        let s_26_4: ProductTypeda0231e9dc169f81 = StageOA(
            state,
            tracer,
            s_26_2,
            s_26_0,
            s_26_1,
            s_26_3,
        );
        // D s_26_5: write-var u#884 <= s_26_4
        fn_state.u_884 = s_26_4;
        // D s_26_6: read-var walkstate.1:struct
        let s_26_6: bool = fn_state.walkstate._1;
        // D s_26_7: cast zx s_26_6 -> bv
        let s_26_7: Bits = Bits::new(s_26_6 as u128, 1u16);
        // C s_26_8: const #1u : u8
        let s_26_8: bool = true;
        // C s_26_9: cast zx s_26_8 -> bv
        let s_26_9: Bits = Bits::new(s_26_8 as u128, 1u16);
        // D s_26_10: cmp-eq s_26_7 s_26_9
        let s_26_10: bool = ((s_26_7) == (s_26_9));
        // N s_26_11: branch s_26_10 b66 b27
        if s_26_10 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#19436 <= s_27_0
        fn_state.gs_19436 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_28_0: read-var gs#19436:u8
        let s_28_0: bool = fn_state.gs_19436;
        // N s_28_1: branch s_28_0 b65 b29
        if s_28_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_29_0: read-var desctype:u32
        let s_29_0: u32 = fn_state.desctype;
        // C s_29_1: const #1u : u32
        let s_29_1: u32 = 1;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // N s_29_3: branch s_29_2 b64 b30
        if s_29_2 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#19467 <= s_30_0
        fn_state.gs_19467 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_31_0: read-var gs#19467:u8
        let s_31_0: bool = fn_state.gs_19467;
        // N s_31_1: branch s_31_0 b63 b32
        if s_31_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#19468 <= s_32_0
        fn_state.gs_19468 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_33_0: read-var gs#19468:u8
        let s_33_0: bool = fn_state.gs_19468;
        // N s_33_1: branch s_33_0 b62 b34
        if s_33_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_34_0: read-var u#884.0:struct
        let s_34_0: u64 = fn_state.u_884._0;
        // D s_34_1: read-var walkparams.2:struct
        let s_34_1: bool = fn_state.walkparams._2;
        // D s_34_2: read-var walkparams.14:struct
        let s_34_2: u8 = fn_state.walkparams._14;
        // D s_34_3: read-var walkparams.26:struct
        let s_34_3: u32 = fn_state.walkparams._26;
        // D s_34_4: call AArch64_OAOutOfRange(s_34_0, s_34_1, s_34_2, s_34_3)
        let s_34_4: bool = AArch64_OAOutOfRange(
            state,
            tracer,
            s_34_0,
            s_34_1,
            s_34_2,
            s_34_3,
        );
        // N s_34_5: branch s_34_4 b61 b35
        if s_34_4 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_35_0: const #10s : i
        let s_35_0: i128 = 10;
        // D s_35_1: read-var descriptor:bv
        let s_35_1: Bits = fn_state.descriptor;
        // C s_35_2: const #1u : u64
        let s_35_2: u64 = 1;
        // D s_35_3: bit-extract s_35_1 s_35_0 s_35_2
        let s_35_3: Bits = (Bits::new(
            ((s_35_1) >> (s_35_0)).value(),
            u16::try_from(s_35_2).unwrap(),
        ));
        // D s_35_4: cast reint s_35_3 -> u8
        let s_35_4: bool = ((s_35_3.value()) != 0);
        // C s_35_5: const #0s : i
        let s_35_5: i128 = 0;
        // C s_35_6: const #0u : u64
        let s_35_6: u64 = 0;
        // D s_35_7: cast zx s_35_4 -> u64
        let s_35_7: u64 = (s_35_4 as u64);
        // C s_35_8: const #1u : u64
        let s_35_8: u64 = 1;
        // D s_35_9: and s_35_7 s_35_8
        let s_35_9: u64 = ((s_35_7) & (s_35_8));
        // D s_35_10: cmp-eq s_35_9 s_35_8
        let s_35_10: bool = ((s_35_9) == (s_35_8));
        // D s_35_11: lsl s_35_7 s_35_5
        let s_35_11: u64 = s_35_7 << s_35_5;
        // D s_35_12: or s_35_6 s_35_11
        let s_35_12: u64 = ((s_35_6) | (s_35_11));
        // D s_35_13: cmpl s_35_11
        let s_35_13: u64 = !s_35_11;
        // D s_35_14: and s_35_6 s_35_13
        let s_35_14: u64 = ((s_35_6) & (s_35_13));
        // D s_35_15: select s_35_10 s_35_12 s_35_14
        let s_35_15: u64 = if s_35_10 { s_35_12 } else { s_35_14 };
        // D s_35_16: cast trunc s_35_15 -> u8
        let s_35_16: bool = ((s_35_15) != 0);
        // D s_35_17: cast zx s_35_16 -> bv
        let s_35_17: Bits = Bits::new(s_35_16 as u128, 1u16);
        // C s_35_18: const #0u : u8
        let s_35_18: bool = false;
        // C s_35_19: cast zx s_35_18 -> bv
        let s_35_19: Bits = Bits::new(s_35_18 as u128, 1u16);
        // D s_35_20: cmp-eq s_35_17 s_35_19
        let s_35_20: bool = ((s_35_17) == (s_35_19));
        // N s_35_21: branch s_35_20 b60 b36
        if s_35_20 {
            return block_60(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#19471 <= s_36_0
        fn_state.gs_19471 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_37_0: read-var gs#19471:u8
        let s_37_0: bool = fn_state.gs_19471;
        // N s_37_1: branch s_37_0 b59 b38
        if s_37_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#19472 <= s_38_0
        fn_state.gs_19472 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_39_0: read-var gs#19472:u8
        let s_39_0: bool = fn_state.gs_19472;
        // N s_39_1: branch s_39_0 b58 b40
        if s_39_0 {
            return block_58(state, tracer, fn_state);
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
        // C s_41_0: const #19088u : u32
        let s_41_0: u32 = 19088;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: bool = {
            let value = state.read_register::<bool>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // N s_41_2: branch s_41_1 b57 b42
        if s_41_1 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#19437 <= s_42_0
        fn_state.gs_19437 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_43_0: read-var gs#19437:u8
        let s_43_0: bool = fn_state.gs_19437;
        // N s_43_1: branch s_43_0 b56 b44
        if s_43_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#19440 <= s_44_0
        fn_state.gs_19440 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_45_0: read-var gs#19440:u8
        let s_45_0: bool = fn_state.gs_19440;
        // N s_45_1: branch s_45_0 b49 b46
        if s_45_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_46_0: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_47_0: read-var fault:struct
        let s_47_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_47_1: read-var walkaddress:struct
        let s_47_1: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_47_2: read-var walkstate:struct
        let s_47_2: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_47_3: read-var descriptor:bv
        let s_47_3: Bits = fn_state.descriptor;
        // D s_47_4: create-product struct = ["s_47_0", "s_47_1", "s_47_2", "s_47_3"]
        let s_47_4: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_47_0,
            _1: s_47_1,
            _2: s_47_2,
            _3: s_47_3,
        };
        // D s_47_5: write-var return_value <= s_47_4
        fn_state.return_value = s_47_4;
        // N s_47_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_48_0: read-var return_value:struct
        let s_48_0: ProductType4b99944cd5e0b59d = fn_state.return_value;
        // N s_48_1: return s_48_0
        return s_48_0;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_49_0: read-var walkstate.7:struct
        let s_49_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_49_1: write-var ga#14709 <= s_49_0
        fn_state.ga_14709 = s_49_0;
        // D s_49_2: read-var ga#14709.7:struct
        let s_49_2: bool = fn_state.ga_14709._7;
        // D s_49_3: write-var tlbcontext.14 <= s_49_2
        fn_state.tlbcontext._14 = s_49_2;
        // D s_49_4: read-var walkstate.6:struct
        let s_49_4: i128 = fn_state.walkstate._6;
        // D s_49_5: write-var tlbcontext.8 <= s_49_4
        fn_state.tlbcontext._8 = s_49_4;
        // D s_49_6: read-var walkparams.2:struct
        let s_49_6: bool = fn_state.walkparams._2;
        // D s_49_7: cast zx s_49_6 -> bv
        let s_49_7: Bits = Bits::new(s_49_6 as u128, 1u16);
        // C s_49_8: const #1u : u8
        let s_49_8: bool = true;
        // C s_49_9: cast zx s_49_8 -> bv
        let s_49_9: Bits = Bits::new(s_49_8 as u128, 1u16);
        // D s_49_10: cmp-eq s_49_7 s_49_9
        let s_49_10: bool = ((s_49_7) == (s_49_9));
        // D s_49_11: write-var tlbcontext.7 <= s_49_10
        fn_state.tlbcontext._7 = s_49_10;
        // D s_49_12: read-var tlbcontext:struct
        let s_49_12: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_49_13: write-var tlbrecord.1 <= s_49_12
        fn_state.tlbrecord._1 = s_49_12;
        // D s_49_14: read-var walkstate:struct
        let s_49_14: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_49_15: write-var tlbrecord.5 <= s_49_14
        fn_state.tlbrecord._5 = s_49_14;
        // D s_49_16: read-var walkparams.2:struct
        let s_49_16: bool = fn_state.walkparams._2;
        // D s_49_17: read-var walkparams.26:struct
        let s_49_17: u32 = fn_state.walkparams._26;
        // D s_49_18: read-var walkstate.6:struct
        let s_49_18: i128 = fn_state.walkstate._6;
        // D s_49_19: call TranslationSize(s_49_16, s_49_17, s_49_18)
        let s_49_19: i128 = TranslationSize(state, tracer, s_49_16, s_49_17, s_49_18);
        // D s_49_20: write-var tlbrecord.0 <= s_49_19
        fn_state.tlbrecord._0 = s_49_19;
        // D s_49_21: read-var walkparams.2:struct
        let s_49_21: bool = fn_state.walkparams._2;
        // D s_49_22: cast zx s_49_21 -> bv
        let s_49_22: Bits = Bits::new(s_49_21 as u128, 1u16);
        // C s_49_23: const #1u : u8
        let s_49_23: bool = true;
        // C s_49_24: cast zx s_49_23 -> bv
        let s_49_24: Bits = Bits::new(s_49_23 as u128, 1u16);
        // D s_49_25: cmp-eq s_49_22 s_49_24
        let s_49_25: bool = ((s_49_22) == (s_49_24));
        // N s_49_26: branch s_49_25 b55 b50
        if s_49_25 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_50_0: read-var Nshadow#321:i64
        let s_50_0: i64 = fn_state.Nshadow_321;
        // D s_50_1: cast zx s_50_0 -> i
        let s_50_1: i128 = (i128::try_from(s_50_0).unwrap());
        // D s_50_2: call __id(s_50_1)
        let s_50_2: i128 = u__id(state, tracer, s_50_1);
        // D s_50_3: cast reint s_50_2 -> i64
        let s_50_3: i64 = (s_50_2 as i64);
        // C s_50_4: const #63s : i
        let s_50_4: i128 = 63;
        // D s_50_5: cast zx s_50_3 -> i
        let s_50_5: i128 = (i128::try_from(s_50_3).unwrap());
        // D s_50_6: cmp-lt s_50_4 s_50_5
        let s_50_6: bool = ((s_50_4) < (s_50_5));
        // N s_50_7: assert s_50_6
        let s_50_7: () = assert!(s_50_6);
        // D s_50_8: read-var tlbrecord:struct
        let s_50_8: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_50_9: write-var tlbrecord <= s_50_8
        fn_state.tlbrecord = s_50_8;
        // D s_50_10: read-var tlbrecord:struct
        let s_50_10: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_50_11: write-var tlbrecord <= s_50_10
        fn_state.tlbrecord = s_50_10;
        // N s_50_12: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_51_0: read-var walkstate.1:struct
        let s_51_0: bool = fn_state.walkstate._1;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #1u : u8
        let s_51_2: bool = true;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // N s_51_5: branch s_51_4 b54 b52
        if s_51_4 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_52_0: const #0s : i
        let s_52_0: i128 = 0;
        // D s_52_1: write-var tlbrecord.2 <= s_52_0
        fn_state.tlbrecord._2 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_53_0: read-var tlbrecord:struct
        let s_53_0: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_53_1: call S2TLBCache(s_53_0)
        let s_53_1: () = S2TLBCache(state, tracer, s_53_0);
        // N s_53_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_54_0: read-var walkparams.2:struct
        let s_54_0: bool = fn_state.walkparams._2;
        // D s_54_1: read-var walkparams.26:struct
        let s_54_1: u32 = fn_state.walkparams._26;
        // D s_54_2: read-var walkstate.6:struct
        let s_54_2: i128 = fn_state.walkstate._6;
        // D s_54_3: call ContiguousSize(s_54_0, s_54_1, s_54_2)
        let s_54_3: i128 = ContiguousSize(state, tracer, s_54_0, s_54_1, s_54_2);
        // D s_54_4: write-var tlbrecord.2 <= s_54_3
        fn_state.tlbrecord._2 = s_54_3;
        // N s_54_5: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_55_0: read-var Nshadow#321:i64
        let s_55_0: i64 = fn_state.Nshadow_321;
        // D s_55_1: cast zx s_55_0 -> i
        let s_55_1: i128 = (i128::try_from(s_55_0).unwrap());
        // D s_55_2: call __id(s_55_1)
        let s_55_2: i128 = u__id(state, tracer, s_55_1);
        // D s_55_3: cast reint s_55_2 -> i64
        let s_55_3: i64 = (s_55_2 as i64);
        // C s_55_4: const #127s : i
        let s_55_4: i128 = 127;
        // D s_55_5: cast zx s_55_3 -> i
        let s_55_5: i128 = (i128::try_from(s_55_3).unwrap());
        // D s_55_6: cmp-lt s_55_4 s_55_5
        let s_55_6: bool = ((s_55_4) < (s_55_5));
        // N s_55_7: assert s_55_6
        let s_55_7: () = assert!(s_55_6);
        // C s_55_8: const #0s : i
        let s_55_8: i128 = 0;
        // D s_55_9: read-var descriptor:bv
        let s_55_9: Bits = fn_state.descriptor;
        // C s_55_10: const #1s : i64
        let s_55_10: i64 = 1;
        // C s_55_11: cast zx s_55_10 -> i
        let s_55_11: i128 = (i128::try_from(s_55_10).unwrap());
        // C s_55_12: const #127s : i
        let s_55_12: i128 = 127;
        // C s_55_13: add s_55_12 s_55_11
        let s_55_13: i128 = (s_55_12 + s_55_11);
        // D s_55_14: bit-extract s_55_9 s_55_8 s_55_13
        let s_55_14: Bits = (Bits::new(
            ((s_55_9) >> (s_55_8)).value(),
            u16::try_from(s_55_13).unwrap(),
        ));
        // D s_55_15: cast reint s_55_14 -> u128
        let s_55_15: u128 = (s_55_14.value() as u128);
        // D s_55_16: write-var tlbrecord.4 <= s_55_15
        fn_state.tlbrecord._4 = s_55_15;
        // N s_55_17: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_56_0: const #10s : i
        let s_56_0: i128 = 10;
        // D s_56_1: read-var descriptor:bv
        let s_56_1: Bits = fn_state.descriptor;
        // C s_56_2: const #1u : u64
        let s_56_2: u64 = 1;
        // D s_56_3: bit-extract s_56_1 s_56_0 s_56_2
        let s_56_3: Bits = (Bits::new(
            ((s_56_1) >> (s_56_0)).value(),
            u16::try_from(s_56_2).unwrap(),
        ));
        // D s_56_4: cast reint s_56_3 -> u8
        let s_56_4: bool = ((s_56_3.value()) != 0);
        // C s_56_5: const #0s : i
        let s_56_5: i128 = 0;
        // C s_56_6: const #0u : u64
        let s_56_6: u64 = 0;
        // D s_56_7: cast zx s_56_4 -> u64
        let s_56_7: u64 = (s_56_4 as u64);
        // C s_56_8: const #1u : u64
        let s_56_8: u64 = 1;
        // D s_56_9: and s_56_7 s_56_8
        let s_56_9: u64 = ((s_56_7) & (s_56_8));
        // D s_56_10: cmp-eq s_56_9 s_56_8
        let s_56_10: bool = ((s_56_9) == (s_56_8));
        // D s_56_11: lsl s_56_7 s_56_5
        let s_56_11: u64 = s_56_7 << s_56_5;
        // D s_56_12: or s_56_6 s_56_11
        let s_56_12: u64 = ((s_56_6) | (s_56_11));
        // D s_56_13: cmpl s_56_11
        let s_56_13: u64 = !s_56_11;
        // D s_56_14: and s_56_6 s_56_13
        let s_56_14: u64 = ((s_56_6) & (s_56_13));
        // D s_56_15: select s_56_10 s_56_12 s_56_14
        let s_56_15: u64 = if s_56_10 { s_56_12 } else { s_56_14 };
        // D s_56_16: cast trunc s_56_15 -> u8
        let s_56_16: bool = ((s_56_15) != 0);
        // D s_56_17: cast zx s_56_16 -> bv
        let s_56_17: Bits = Bits::new(s_56_16 as u128, 1u16);
        // C s_56_18: const #1u : u8
        let s_56_18: bool = true;
        // C s_56_19: cast zx s_56_18 -> bv
        let s_56_19: Bits = Bits::new(s_56_18 as u128, 1u16);
        // D s_56_20: cmp-eq s_56_17 s_56_19
        let s_56_20: bool = ((s_56_17) == (s_56_19));
        // D s_56_21: write-var gs#19440 <= s_56_20
        fn_state.gs_19440 = s_56_20;
        // N s_56_22: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_57_0: read-var fault.16:struct
        let s_57_0: u32 = fn_state.fault._16;
        // C s_57_1: const #0u : u32
        let s_57_1: u32 = 0;
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // D s_57_3: write-var gs#19437 <= s_57_2
        fn_state.gs_19437 = s_57_2;
        // N s_57_4: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_58_0: const #1u : u32
        let s_58_0: u32 = 1;
        // D s_58_1: write-var fault.16 <= s_58_0
        fn_state.fault._16 = s_58_0;
        // N s_58_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var gs#19472 <= s_59_0
        fn_state.gs_19472 = s_59_0;
        // N s_59_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_60_0: read-var walkparams.7:struct
        let s_60_0: bool = fn_state.walkparams._7;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #0u : u8
        let s_60_2: bool = false;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#19471 <= s_60_4
        fn_state.gs_19471 = s_60_4;
        // N s_60_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_61_0: const #7u : u32
        let s_61_0: u32 = 7;
        // D s_61_1: write-var fault.16 <= s_61_0
        fn_state.fault._16 = s_61_0;
        // N s_61_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_62_0: const #6u : u32
        let s_62_0: u32 = 6;
        // D s_62_1: write-var fault.16 <= s_62_0
        fn_state.fault._16 = s_62_0;
        // N s_62_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_63_0: read-var walkparams.2:struct
        let s_63_0: bool = fn_state.walkparams._2;
        // D s_63_1: read-var descriptor:bv
        let s_63_1: Bits = fn_state.descriptor;
        // D s_63_2: call AArch64_BlocknTFaults(s_63_0, s_63_1)
        let s_63_2: bool = AArch64_BlocknTFaults(state, tracer, s_63_0, s_63_1);
        // D s_63_3: write-var gs#19468 <= s_63_2
        fn_state.gs_19468 = s_63_2;
        // N s_63_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_64_0: read-var walkstate.6:struct
        let s_64_0: i128 = fn_state.walkstate._6;
        // C s_64_1: const #800u : u32
        let s_64_1: u32 = 800;
        // D s_64_2: read-reg s_64_1:i64
        let s_64_2: i64 = {
            let value = state.read_register::<i64>(s_64_1 as isize);
            tracer.read_register(s_64_1 as isize, value);
            value
        };
        // D s_64_3: cast zx s_64_2 -> i
        let s_64_3: i128 = (i128::try_from(s_64_2).unwrap());
        // D s_64_4: cmp-lt s_64_0 s_64_3
        let s_64_4: bool = ((s_64_0) < (s_64_3));
        // D s_64_5: write-var gs#19467 <= s_64_4
        fn_state.gs_19467 = s_64_4;
        // N s_64_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_65_0: const #6u : u32
        let s_65_0: u32 = 6;
        // D s_65_1: write-var fault.16 <= s_65_0
        fn_state.fault._16 = s_65_0;
        // N s_65_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_66_0: read-var walkparams.2:struct
        let s_66_0: bool = fn_state.walkparams._2;
        // D s_66_1: read-var walkparams.29:struct
        let s_66_1: u8 = fn_state.walkparams._29;
        // D s_66_2: read-var walkparams.26:struct
        let s_66_2: u32 = fn_state.walkparams._26;
        // D s_66_3: read-var walkstate.6:struct
        let s_66_3: i128 = fn_state.walkstate._6;
        // D s_66_4: call AArch64_ContiguousBitFaults(s_66_0, s_66_1, s_66_2, s_66_3)
        let s_66_4: bool = AArch64_ContiguousBitFaults(
            state,
            tracer,
            s_66_0,
            s_66_1,
            s_66_2,
            s_66_3,
        );
        // D s_66_5: write-var gs#19436 <= s_66_4
        fn_state.gs_19436 = s_66_4;
        // N s_66_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_67_0: read-var accdesc:struct
        let s_67_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_67_1: call CreateAccDescTTEUpdate(s_67_0)
        let s_67_1: ProductType9878976b5bcce9c9 = CreateAccDescTTEUpdate(
            state,
            tracer,
            s_67_0,
        );
        // D s_67_2: write-var descaccess <= s_67_1
        fn_state.descaccess = s_67_1;
        // D s_67_3: read-var Nshadow#321:i64
        let s_67_3: i64 = fn_state.Nshadow_321;
        // D s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // D s_67_5: call __id(s_67_4)
        let s_67_5: i128 = u__id(state, tracer, s_67_4);
        // D s_67_6: cast reint s_67_5 -> i64
        let s_67_6: i64 = (s_67_5 as i64);
        // C s_67_7: const #64s : i
        let s_67_7: i128 = 64;
        // D s_67_8: cast zx s_67_6 -> i
        let s_67_8: i128 = (i128::try_from(s_67_6).unwrap());
        // D s_67_9: cmp-eq s_67_8 s_67_7
        let s_67_9: bool = ((s_67_8) == (s_67_7));
        // N s_67_10: branch s_67_9 b72 b68
        if s_67_9 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_68_0: read-var Nshadow#321:i64
        let s_68_0: i64 = fn_state.Nshadow_321;
        // D s_68_1: cast zx s_68_0 -> i
        let s_68_1: i128 = (i128::try_from(s_68_0).unwrap());
        // D s_68_2: call __id(s_68_1)
        let s_68_2: i128 = u__id(state, tracer, s_68_1);
        // D s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: const #128s : i
        let s_68_4: i128 = 128;
        // D s_68_5: cast zx s_68_3 -> i
        let s_68_5: i128 = (i128::try_from(s_68_3).unwrap());
        // D s_68_6: cmp-eq s_68_5 s_68_4
        let s_68_6: bool = ((s_68_5) == (s_68_4));
        // D s_68_7: write-var gs#19415 <= s_68_6
        fn_state.gs_19415 = s_68_6;
        // N s_68_8: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_69_0: read-var gs#19415:u8
        let s_69_0: bool = fn_state.gs_19415;
        // N s_69_1: assert s_69_0
        let s_69_1: () = assert!(s_69_0);
        // D s_69_2: read-var walkparams.4:struct
        let s_69_2: bool = fn_state.walkparams._4;
        // D s_69_3: read-var fault:struct
        let s_69_3: ProductType1d757adad216cdef = fn_state.fault;
        // D s_69_4: read-var descriptor:bv
        let s_69_4: Bits = fn_state.descriptor;
        // D s_69_5: read-var new_descriptor:bv
        let s_69_5: Bits = fn_state.new_descriptor;
        // D s_69_6: read-var descaccess:struct
        let s_69_6: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_69_7: read-var walkaddress:struct
        let s_69_7: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_69_8: read-var translation_info:struct
        let s_69_8: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_69_9: call AArch64_MemSwapTableDesc(s_69_3, s_69_4, s_69_5, s_69_2, s_69_6, s_69_7, s_69_8)
        let s_69_9: ProductTypeb4cea7287e2eb9d6 = AArch64_MemSwapTableDesc(
            state,
            tracer,
            s_69_3,
            s_69_4,
            s_69_5,
            s_69_2,
            s_69_6,
            s_69_7,
            s_69_8,
        );
        // D s_69_10: write-var ga#14663 <= s_69_9
        fn_state.ga_14663 = s_69_9;
        // D s_69_11: read-var ga#14663.0:struct
        let s_69_11: ProductType1d757adad216cdef = fn_state.ga_14663._0;
        // D s_69_12: read-var ga#14663.1:struct
        let s_69_12: Bits = fn_state.ga_14663._1;
        // D s_69_13: write-var fault <= s_69_11
        fn_state.fault = s_69_11;
        // D s_69_14: write-var descriptor <= s_69_12
        fn_state.descriptor = s_69_12;
        // D s_69_15: read-var fault.16:struct
        let s_69_15: u32 = fn_state.fault._16;
        // C s_69_16: const #0u : u32
        let s_69_16: u32 = 0;
        // D s_69_17: cmp-eq s_69_15 s_69_16
        let s_69_17: bool = ((s_69_15) == (s_69_16));
        // N s_69_18: branch s_69_17 b71 b70
        if s_69_17 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_70_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call __UNKNOWN_AddressDescriptor(s_71_0)
        let s_71_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_71_0,
        );
        // C s_71_2: const #() : ()
        let s_71_2: () = ();
        // S s_71_3: call __UNKNOWN_TTWState(s_71_2)
        let s_71_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_71_2,
        );
        // D s_71_4: read-var Nshadow#321:i64
        let s_71_4: i64 = fn_state.Nshadow_321;
        // D s_71_5: cast zx s_71_4 -> i
        let s_71_5: i128 = (i128::try_from(s_71_4).unwrap());
        // D s_71_6: cast reint s_71_5 -> i64
        let s_71_6: i64 = (s_71_5 as i64);
        // D s_71_7: cast zx s_71_6 -> i
        let s_71_7: i128 = (i128::try_from(s_71_6).unwrap());
        // D s_71_8: call __UNKNOWN_bits(s_71_7)
        let s_71_8: Bits = u__UNKNOWN_bits(state, tracer, s_71_7);
        // D s_71_9: read-var fault:struct
        let s_71_9: ProductType1d757adad216cdef = fn_state.fault;
        // D s_71_10: create-product struct = ["s_71_9", "s_71_1", "s_71_3", "s_71_8"]
        let s_71_10: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_71_9,
            _1: s_71_1,
            _2: s_71_3,
            _3: s_71_8,
        };
        // D s_71_11: write-var return_value <= s_71_10
        fn_state.return_value = s_71_10;
        // N s_71_12: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#19415 <= s_72_0
        fn_state.gs_19415 = s_72_0;
        // N s_72_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_73_0: const #6u : u32
        let s_73_0: u32 = 6;
        // D s_73_1: write-var fault.16 <= s_73_0
        fn_state.fault._16 = s_73_0;
        // C s_73_2: const #() : ()
        let s_73_2: () = ();
        // S s_73_3: call __UNKNOWN_AddressDescriptor(s_73_2)
        let s_73_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_73_2,
        );
        // C s_73_4: const #() : ()
        let s_73_4: () = ();
        // S s_73_5: call __UNKNOWN_TTWState(s_73_4)
        let s_73_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_73_4,
        );
        // D s_73_6: read-var Nshadow#321:i64
        let s_73_6: i64 = fn_state.Nshadow_321;
        // D s_73_7: cast zx s_73_6 -> i
        let s_73_7: i128 = (i128::try_from(s_73_6).unwrap());
        // D s_73_8: cast reint s_73_7 -> i64
        let s_73_8: i64 = (s_73_7 as i64);
        // D s_73_9: cast zx s_73_8 -> i
        let s_73_9: i128 = (i128::try_from(s_73_8).unwrap());
        // D s_73_10: call __UNKNOWN_bits(s_73_9)
        let s_73_10: Bits = u__UNKNOWN_bits(state, tracer, s_73_9);
        // D s_73_11: read-var fault:struct
        let s_73_11: ProductType1d757adad216cdef = fn_state.fault;
        // D s_73_12: create-product struct = ["s_73_11", "s_73_3", "s_73_5", "s_73_10"]
        let s_73_12: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_73_11,
            _1: s_73_3,
            _2: s_73_5,
            _3: s_73_10,
        };
        // D s_73_13: write-var return_value <= s_73_12
        fn_state.return_value = s_73_12;
        // N s_73_14: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_74_0: read-var walkparams.2:struct
        let s_74_0: bool = fn_state.walkparams._2;
        // D s_74_1: read-var descriptor:bv
        let s_74_1: Bits = fn_state.descriptor;
        // D s_74_2: call AArch64_BlocknTFaults(s_74_0, s_74_1)
        let s_74_2: bool = AArch64_BlocknTFaults(state, tracer, s_74_0, s_74_1);
        // D s_74_3: write-var gs#19392 <= s_74_2
        fn_state.gs_19392 = s_74_2;
        // N s_74_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_75_0: read-var skl:u8
        let s_75_0: u8 = fn_state.skl;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 2u16);
        // C s_75_2: const #0u : u8
        let s_75_2: u8 = 0;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 2u16);
        // D s_75_4: cmp-ne s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) != (s_75_3));
        // D s_75_5: write-var gs#19391 <= s_75_4
        fn_state.gs_19391 = s_75_4;
        // N s_75_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_76_0: const #10s : i
        let s_76_0: i128 = 10;
        // D s_76_1: read-var new_descriptor:bv
        let s_76_1: Bits = fn_state.new_descriptor;
        // C s_76_2: const #1u : u64
        let s_76_2: u64 = 1;
        // D s_76_3: bit-insert s_76_1 s_76_1 s_76_0 s_76_2
        let s_76_3: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_76_2 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_76_1.length(),
            );
            (s_76_1 & mask) | (s_76_1 << s_76_0)
        };
        // D s_76_4: write-var new_descriptor <= s_76_3
        fn_state.new_descriptor = s_76_3;
        // N s_76_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_77_0: const #7u : u32
        let s_77_0: u32 = 7;
        // D s_77_1: write-var fault.16 <= s_77_0
        fn_state.fault._16 = s_77_0;
        // C s_77_2: const #() : ()
        let s_77_2: () = ();
        // S s_77_3: call __UNKNOWN_AddressDescriptor(s_77_2)
        let s_77_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_77_2,
        );
        // C s_77_4: const #() : ()
        let s_77_4: () = ();
        // S s_77_5: call __UNKNOWN_TTWState(s_77_4)
        let s_77_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_77_4,
        );
        // D s_77_6: read-var Nshadow#321:i64
        let s_77_6: i64 = fn_state.Nshadow_321;
        // D s_77_7: cast zx s_77_6 -> i
        let s_77_7: i128 = (i128::try_from(s_77_6).unwrap());
        // D s_77_8: cast reint s_77_7 -> i64
        let s_77_8: i64 = (s_77_7 as i64);
        // D s_77_9: cast zx s_77_8 -> i
        let s_77_9: i128 = (i128::try_from(s_77_8).unwrap());
        // D s_77_10: call __UNKNOWN_bits(s_77_9)
        let s_77_10: Bits = u__UNKNOWN_bits(state, tracer, s_77_9);
        // D s_77_11: read-var fault:struct
        let s_77_11: ProductType1d757adad216cdef = fn_state.fault;
        // D s_77_12: create-product struct = ["s_77_11", "s_77_3", "s_77_5", "s_77_10"]
        let s_77_12: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_77_11,
            _1: s_77_3,
            _2: s_77_5,
            _3: s_77_10,
        };
        // D s_77_13: write-var return_value <= s_77_12
        fn_state.return_value = s_77_12;
        // N s_77_14: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_78_0: read-var Nshadow#321:i64
        let s_78_0: i64 = fn_state.Nshadow_321;
        // D s_78_1: cast zx s_78_0 -> i
        let s_78_1: i128 = (i128::try_from(s_78_0).unwrap());
        // D s_78_2: call __id(s_78_1)
        let s_78_2: i128 = u__id(state, tracer, s_78_1);
        // D s_78_3: cast reint s_78_2 -> i64
        let s_78_3: i64 = (s_78_2 as i64);
        // C s_78_4: const #110s : i
        let s_78_4: i128 = 110;
        // D s_78_5: cast zx s_78_3 -> i
        let s_78_5: i128 = (i128::try_from(s_78_3).unwrap());
        // D s_78_6: cmp-lt s_78_4 s_78_5
        let s_78_6: bool = ((s_78_4) < (s_78_5));
        // N s_78_7: assert s_78_6
        let s_78_7: () = assert!(s_78_6);
        // C s_78_8: const #109s : i
        let s_78_8: i128 = 109;
        // D s_78_9: read-var descriptor:bv
        let s_78_9: Bits = fn_state.descriptor;
        // C s_78_10: const #1s : i64
        let s_78_10: i64 = 1;
        // C s_78_11: cast zx s_78_10 -> i
        let s_78_11: i128 = (i128::try_from(s_78_10).unwrap());
        // C s_78_12: const #1s : i
        let s_78_12: i128 = 1;
        // C s_78_13: add s_78_12 s_78_11
        let s_78_13: i128 = (s_78_12 + s_78_11);
        // D s_78_14: bit-extract s_78_9 s_78_8 s_78_13
        let s_78_14: Bits = (Bits::new(
            ((s_78_9) >> (s_78_8)).value(),
            u16::try_from(s_78_13).unwrap(),
        ));
        // D s_78_15: cast reint s_78_14 -> u8
        let s_78_15: u8 = (s_78_14.value() as u8);
        // D s_78_16: write-var skl <= s_78_15
        fn_state.skl = s_78_15;
        // N s_78_17: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_79_0: const #1u : u32
        let s_79_0: u32 = 1;
        // D s_79_1: read-var desctype:u32
        let s_79_1: u32 = fn_state.desctype;
        // D s_79_2: cmp-eq s_79_0 s_79_1
        let s_79_2: bool = ((s_79_0) == (s_79_1));
        // D s_79_3: not s_79_2
        let s_79_3: bool = !s_79_2;
        // N s_79_4: branch s_79_3 b81 b80
        if s_79_3 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_80_0: read-var Nshadow#321:i64
        let s_80_0: i64 = fn_state.Nshadow_321;
        // D s_80_1: cast zx s_80_0 -> i
        let s_80_1: i128 = (i128::try_from(s_80_0).unwrap());
        // D s_80_2: call __id(s_80_1)
        let s_80_2: i128 = u__id(state, tracer, s_80_1);
        // D s_80_3: cast reint s_80_2 -> i64
        let s_80_3: i64 = (s_80_2 as i64);
        // C s_80_4: const #52s : i
        let s_80_4: i128 = 52;
        // D s_80_5: cast zx s_80_3 -> i
        let s_80_5: i128 = (i128::try_from(s_80_3).unwrap());
        // D s_80_6: cmp-lt s_80_4 s_80_5
        let s_80_6: bool = ((s_80_4) < (s_80_5));
        // N s_80_7: assert s_80_6
        let s_80_7: () = assert!(s_80_6);
        // D s_80_8: read-var accdesc.25:struct
        let s_80_8: u32 = fn_state.accdesc._25;
        // D s_80_9: read-var walkstate:struct
        let s_80_9: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_80_10: read-var walkparams:struct
        let s_80_10: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_80_11: read-var ipa:struct
        let s_80_11: ProductTypece7c66ccb2cab13e = fn_state.ipa;
        // D s_80_12: read-var descriptor:bv
        let s_80_12: Bits = fn_state.descriptor;
        // D s_80_13: call AArch64_S2NextWalkStateLeaf(s_80_9, s_80_8, s_80_10, s_80_11, s_80_12)
        let s_80_13: ProductType96e7acababe246a1 = AArch64_S2NextWalkStateLeaf(
            state,
            tracer,
            s_80_9,
            s_80_8,
            s_80_10,
            s_80_11,
            s_80_12,
        );
        // D s_80_14: write-var walkstate <= s_80_13
        fn_state.walkstate = s_80_13;
        // N s_80_15: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_81_0: const #2u : u32
        let s_81_0: u32 = 2;
        // D s_81_1: read-var desctype:u32
        let s_81_1: u32 = fn_state.desctype;
        // D s_81_2: cmp-eq s_81_0 s_81_1
        let s_81_2: bool = ((s_81_0) == (s_81_1));
        // D s_81_3: not s_81_2
        let s_81_3: bool = !s_81_2;
        // N s_81_4: branch s_81_3 b83 b82
        if s_81_3 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_82_0: const #6u : u32
        let s_82_0: u32 = 6;
        // D s_82_1: write-var fault.16 <= s_82_0
        fn_state.fault._16 = s_82_0;
        // C s_82_2: const #() : ()
        let s_82_2: () = ();
        // S s_82_3: call __UNKNOWN_AddressDescriptor(s_82_2)
        let s_82_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_82_2,
        );
        // C s_82_4: const #() : ()
        let s_82_4: () = ();
        // S s_82_5: call __UNKNOWN_TTWState(s_82_4)
        let s_82_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_82_4,
        );
        // D s_82_6: read-var Nshadow#321:i64
        let s_82_6: i64 = fn_state.Nshadow_321;
        // D s_82_7: cast zx s_82_6 -> i
        let s_82_7: i128 = (i128::try_from(s_82_6).unwrap());
        // D s_82_8: cast reint s_82_7 -> i64
        let s_82_8: i64 = (s_82_7 as i64);
        // D s_82_9: cast zx s_82_8 -> i
        let s_82_9: i128 = (i128::try_from(s_82_8).unwrap());
        // D s_82_10: call __UNKNOWN_bits(s_82_9)
        let s_82_10: Bits = u__UNKNOWN_bits(state, tracer, s_82_9);
        // D s_82_11: read-var fault:struct
        let s_82_11: ProductType1d757adad216cdef = fn_state.fault;
        // D s_82_12: create-product struct = ["s_82_11", "s_82_3", "s_82_5", "s_82_10"]
        let s_82_12: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_82_11,
            _1: s_82_3,
            _2: s_82_5,
            _3: s_82_10,
        };
        // D s_82_13: write-var return_value <= s_82_12
        fn_state.return_value = s_82_12;
        // N s_82_14: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call Unreachable(s_83_0)
        let s_83_1: () = Unreachable(state, tracer, s_83_0);
        // N s_83_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call __UNKNOWN_AddressDescriptor(s_84_0)
        let s_84_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_84_0,
        );
        // C s_84_2: const #() : ()
        let s_84_2: () = ();
        // S s_84_3: call __UNKNOWN_TTWState(s_84_2)
        let s_84_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_84_2,
        );
        // D s_84_4: read-var Nshadow#321:i64
        let s_84_4: i64 = fn_state.Nshadow_321;
        // D s_84_5: cast zx s_84_4 -> i
        let s_84_5: i128 = (i128::try_from(s_84_4).unwrap());
        // D s_84_6: cast reint s_84_5 -> i64
        let s_84_6: i64 = (s_84_5 as i64);
        // D s_84_7: cast zx s_84_6 -> i
        let s_84_7: i128 = (i128::try_from(s_84_6).unwrap());
        // D s_84_8: call __UNKNOWN_bits(s_84_7)
        let s_84_8: Bits = u__UNKNOWN_bits(state, tracer, s_84_7);
        // D s_84_9: read-var fault:struct
        let s_84_9: ProductType1d757adad216cdef = fn_state.fault;
        // D s_84_10: create-product struct = ["s_84_9", "s_84_1", "s_84_3", "s_84_8"]
        let s_84_10: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_84_9,
            _1: s_84_1,
            _2: s_84_3,
            _3: s_84_8,
        };
        // D s_84_11: write-var return_value <= s_84_10
        fn_state.return_value = s_84_10;
        // N s_84_12: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_85_0: const #7u : u32
        let s_85_0: u32 = 7;
        // D s_85_1: write-var fault.16 <= s_85_0
        fn_state.fault._16 = s_85_0;
        // C s_85_2: const #0s : i
        let s_85_2: i128 = 0;
        // D s_85_3: write-var fault.9 <= s_85_2
        fn_state.fault._9 = s_85_2;
        // C s_85_4: const #() : ()
        let s_85_4: () = ();
        // S s_85_5: call __UNKNOWN_AddressDescriptor(s_85_4)
        let s_85_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_85_4,
        );
        // C s_85_6: const #() : ()
        let s_85_6: () = ();
        // S s_85_7: call __UNKNOWN_TTWState(s_85_6)
        let s_85_7: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_85_6,
        );
        // D s_85_8: read-var Nshadow#321:i64
        let s_85_8: i64 = fn_state.Nshadow_321;
        // D s_85_9: cast zx s_85_8 -> i
        let s_85_9: i128 = (i128::try_from(s_85_8).unwrap());
        // D s_85_10: cast reint s_85_9 -> i64
        let s_85_10: i64 = (s_85_9 as i64);
        // D s_85_11: cast zx s_85_10 -> i
        let s_85_11: i128 = (i128::try_from(s_85_10).unwrap());
        // D s_85_12: call __UNKNOWN_bits(s_85_11)
        let s_85_12: Bits = u__UNKNOWN_bits(state, tracer, s_85_11);
        // D s_85_13: read-var fault:struct
        let s_85_13: ProductType1d757adad216cdef = fn_state.fault;
        // D s_85_14: create-product struct = ["s_85_13", "s_85_5", "s_85_7", "s_85_12"]
        let s_85_14: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_85_13,
            _1: s_85_5,
            _2: s_85_7,
            _3: s_85_12,
        };
        // D s_85_15: write-var return_value <= s_85_14
        fn_state.return_value = s_85_14;
        // N s_85_16: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call NormalNCMemAttr(s_86_0)
        let s_86_1: ProductTypef170cab34335b70c = NormalNCMemAttr(state, tracer, s_86_0);
        // D s_86_2: write-var walkaddress.2 <= s_86_1
        fn_state.walkaddress._2 = s_86_1;
        // D s_86_3: read-var walkstate.7:struct
        let s_86_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_86_4: write-var ga#14592 <= s_86_3
        fn_state.ga_14592 = s_86_3;
        // D s_86_5: read-var ga#14592.7:struct
        let s_86_5: bool = fn_state.ga_14592._7;
        // D s_86_6: write-var walkaddress.2.7 <= s_86_5
        fn_state.walkaddress._2._7 = s_86_5;
        // N s_86_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_87_0: read-var ipa.3:struct
        let s_87_0: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_87_1: write-var ga#14584 <= s_87_0
        fn_state.ga_14584 = s_87_0;
        // D s_87_2: read-var ga#14584.1:struct
        let s_87_2: u32 = fn_state.ga_14584._1;
        // D s_87_3: read-var walkparams:struct
        let s_87_3: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_87_4: call AArch64_SS2InitialTTWState(s_87_3, s_87_2)
        let s_87_4: ProductType96e7acababe246a1 = AArch64_SS2InitialTTWState(
            state,
            tracer,
            s_87_3,
            s_87_2,
        );
        // D s_87_5: write-var walkstate <= s_87_4
        fn_state.walkstate = s_87_4;
        // N s_87_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_88_0: read-var accdesc.25:struct
        let s_88_0: u32 = fn_state.accdesc._25;
        // D s_88_1: read-var ipa.3:struct
        let s_88_1: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_88_2: read-var walkparams.26:struct
        let s_88_2: u32 = fn_state.walkparams._26;
        // D s_88_3: call AArch64_GetS2TLBContext(s_88_0, s_88_1, s_88_2)
        let s_88_3: ProductTypec0d0fb0603850c4c = AArch64_GetS2TLBContext(
            state,
            tracer,
            s_88_0,
            s_88_1,
            s_88_2,
        );
        // D s_88_4: write-var tlbcontext <= s_88_3
        fn_state.tlbcontext = s_88_3;
        // D s_88_5: read-var tlbcontext:struct
        let s_88_5: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_88_6: call S2TLBLookup(s_88_5)
        let s_88_6: ProductTypeeb828c17bbe5e68 = S2TLBLookup(state, tracer, s_88_5);
        // D s_88_7: write-var tlbentry <= s_88_6
        fn_state.tlbentry = s_88_6;
        // D s_88_8: read-var tlbentry.0:struct
        let s_88_8: ProductTypee47dd77b186df56e = fn_state.tlbentry._0;
        // D s_88_9: write-var tlbrecord <= s_88_8
        fn_state.tlbrecord = s_88_8;
        // D s_88_10: read-var walkparams.2:struct
        let s_88_10: bool = fn_state.walkparams._2;
        // D s_88_11: read-var walkparams.26:struct
        let s_88_11: u32 = fn_state.walkparams._26;
        // D s_88_12: read-var tlbrecord.5:struct
        let s_88_12: ProductType96e7acababe246a1 = fn_state.tlbrecord._5;
        // D s_88_13: read-var ipa_64:u64
        let s_88_13: u64 = fn_state.ipa_64;
        // D s_88_14: call StageOA(s_88_13, s_88_10, s_88_11, s_88_12)
        let s_88_14: ProductTypeda0231e9dc169f81 = StageOA(
            state,
            tracer,
            s_88_13,
            s_88_10,
            s_88_11,
            s_88_12,
        );
        // D s_88_15: write-var oa <= s_88_14
        fn_state.oa = s_88_14;
        // D s_88_16: read-var tlbentry.1:struct
        let s_88_16: bool = fn_state.tlbentry._1;
        // N s_88_17: branch s_88_16 b122 b89
        if s_88_16 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#19347 <= s_89_0
        fn_state.gs_19347 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_90_0: read-var gs#19347:u8
        let s_90_0: bool = fn_state.gs_19347;
        // N s_90_1: branch s_90_0 b100 b91
        if s_90_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#19359 <= s_91_0
        fn_state.gs_19359 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_92_0: read-var gs#19359:u8
        let s_92_0: bool = fn_state.gs_19359;
        // N s_92_1: branch s_92_0 b94 b93
        if s_92_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_93_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_94_0: read-var walkparams.6:struct
        let s_94_0: bool = fn_state.walkparams._6;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #1u : u8
        let s_94_2: bool = true;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // N s_94_5: branch s_94_4 b97 b95
        if s_94_4 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // N s_95_0: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_96_0: read-var tlbrecord.5:struct
        let s_96_0: ProductType96e7acababe246a1 = fn_state.tlbrecord._5;
        // D s_96_1: write-var ga#14572 <= s_96_0
        fn_state.ga_14572 = s_96_0;
        // D s_96_2: read-var ga#14572.6:struct
        let s_96_2: i128 = fn_state.ga_14572._6;
        // D s_96_3: write-var fault.9 <= s_96_2
        fn_state.fault._9 = s_96_2;
        // C s_96_4: const #() : ()
        let s_96_4: () = ();
        // S s_96_5: call __UNKNOWN_AddressDescriptor(s_96_4)
        let s_96_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_96_4,
        );
        // D s_96_6: read-var tlbrecord.5:struct
        let s_96_6: ProductType96e7acababe246a1 = fn_state.tlbrecord._5;
        // D s_96_7: read-var tlbrecord.4:struct
        let s_96_7: u128 = fn_state.tlbrecord._4;
        // C s_96_8: const #1s : i
        let s_96_8: i128 = 1;
        // D s_96_9: read-var Nshadow#321:i64
        let s_96_9: i64 = fn_state.Nshadow_321;
        // D s_96_10: cast zx s_96_9 -> i
        let s_96_10: i128 = (i128::try_from(s_96_9).unwrap());
        // D s_96_11: sub s_96_10 s_96_8
        let s_96_11: i128 = ((s_96_10) - (s_96_8));
        // D s_96_12: cast reint s_96_11 -> i64
        let s_96_12: i64 = (s_96_11 as i64);
        // C s_96_13: const #0s : i
        let s_96_13: i128 = 0;
        // D s_96_14: cast zx s_96_7 -> bv
        let s_96_14: Bits = Bits::new(s_96_7 as u128, 128u16);
        // D s_96_15: cast zx s_96_12 -> i
        let s_96_15: i128 = (i128::try_from(s_96_12).unwrap());
        // C s_96_16: const #1s : i64
        let s_96_16: i64 = 1;
        // C s_96_17: cast zx s_96_16 -> i
        let s_96_17: i128 = (i128::try_from(s_96_16).unwrap());
        // D s_96_18: sub s_96_15 s_96_13
        let s_96_18: i128 = ((s_96_15) - (s_96_13));
        // D s_96_19: add s_96_18 s_96_17
        let s_96_19: i128 = (s_96_18 + s_96_17);
        // D s_96_20: bit-extract s_96_14 s_96_13 s_96_19
        let s_96_20: Bits = (Bits::new(
            ((s_96_14) >> (s_96_13)).value(),
            u16::try_from(s_96_19).unwrap(),
        ));
        // D s_96_21: read-var fault:struct
        let s_96_21: ProductType1d757adad216cdef = fn_state.fault;
        // D s_96_22: create-product struct = ["s_96_21", "s_96_5", "s_96_6", "s_96_20"]
        let s_96_22: ProductType4b99944cd5e0b59d = ProductType4b99944cd5e0b59d {
            _0: s_96_21,
            _1: s_96_5,
            _2: s_96_6,
            _3: s_96_20,
        };
        // D s_96_23: write-var return_value <= s_96_22
        fn_state.return_value = s_96_22;
        // N s_96_24: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_97_0: read-var ipa.2:struct
        let s_97_0: ProductTypef170cab34335b70c = fn_state.ipa._2;
        // D s_97_1: read-var tlbrecord.4:struct
        let s_97_1: u128 = fn_state.tlbrecord._4;
        // D s_97_2: cast zx s_97_1 -> bv
        let s_97_2: Bits = Bits::new(s_97_1 as u128, 128u16);
        // D s_97_3: read-var walkparams:struct
        let s_97_3: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_97_4: call AArch64_S2ApplyFWBMemAttrs(s_97_0, s_97_3, s_97_2)
        let s_97_4: ProductTypef170cab34335b70c = AArch64_S2ApplyFWBMemAttrs(
            state,
            tracer,
            s_97_0,
            s_97_3,
            s_97_2,
        );
        // D s_97_5: write-var tlbrecord.5.7 <= s_97_4
        fn_state.tlbrecord._5._7 = s_97_4;
        // D s_97_6: read-var tlbrecord.4:struct
        let s_97_6: u128 = fn_state.tlbrecord._4;
        // C s_97_7: const #3s : i
        let s_97_7: i128 = 3;
        // D s_97_8: cast zx s_97_6 -> bv
        let s_97_8: Bits = Bits::new(s_97_6 as u128, 128u16);
        // C s_97_9: const #1s : i64
        let s_97_9: i64 = 1;
        // C s_97_10: cast zx s_97_9 -> i
        let s_97_10: i128 = (i128::try_from(s_97_9).unwrap());
        // C s_97_11: const #2s : i
        let s_97_11: i128 = 2;
        // C s_97_12: add s_97_11 s_97_10
        let s_97_12: i128 = (s_97_11 + s_97_10);
        // D s_97_13: bit-extract s_97_8 s_97_7 s_97_12
        let s_97_13: Bits = (Bits::new(
            ((s_97_8) >> (s_97_7)).value(),
            u16::try_from(s_97_12).unwrap(),
        ));
        // D s_97_14: cast reint s_97_13 -> u8
        let s_97_14: u8 = (s_97_13.value() as u8);
        // D s_97_15: cast zx s_97_14 -> bv
        let s_97_15: Bits = Bits::new(s_97_14 as u128, 3u16);
        // C s_97_16: const #7u : u8
        let s_97_16: u8 = 7;
        // C s_97_17: cast zx s_97_16 -> bv
        let s_97_17: Bits = Bits::new(s_97_16 as u128, 3u16);
        // D s_97_18: cmp-eq s_97_15 s_97_17
        let s_97_18: bool = ((s_97_15) == (s_97_17));
        // N s_97_19: branch s_97_18 b99 b98
        if s_97_18 {
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
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var tlbrecord.5.9.11 <= s_98_0
        fn_state.tlbrecord._5._9._11 = s_98_0;
        // N s_98_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_99_0: const #1u : u8
        let s_99_0: bool = true;
        // D s_99_1: write-var tlbrecord.5.9.11 <= s_99_0
        fn_state.tlbrecord._5._9._11 = s_99_0;
        // N s_99_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_100_0: read-var walkparams.7:struct
        let s_100_0: bool = fn_state.walkparams._7;
        // D s_100_1: read-var walkparams.9:struct
        let s_100_1: bool = fn_state.walkparams._9;
        // D s_100_2: cast zx s_100_0 -> bv
        let s_100_2: Bits = Bits::new(s_100_0 as u128, 1u16);
        // D s_100_3: cast zx s_100_1 -> bv
        let s_100_3: Bits = Bits::new(s_100_1 as u128, 1u16);
        // D s_100_4: cast reint s_100_2 -> u128
        let s_100_4: u128 = (s_100_2.value() as u128);
        // D s_100_5: size-of s_100_2
        let s_100_5: u16 = s_100_2.length();
        // D s_100_6: cast reint s_100_3 -> u128
        let s_100_6: u128 = (s_100_3.value() as u128);
        // D s_100_7: size-of s_100_3
        let s_100_7: u16 = s_100_3.length();
        // D s_100_8: lsl s_100_4 s_100_7
        let s_100_8: u128 = s_100_4 << s_100_7;
        // D s_100_9: or s_100_8 s_100_6
        let s_100_9: u128 = ((s_100_8) | (s_100_6));
        // D s_100_10: add s_100_5 s_100_7
        let s_100_10: u16 = (s_100_5 + s_100_7);
        // D s_100_11: create-bits s_100_9 s_100_10
        let s_100_11: Bits = Bits::new(s_100_9, s_100_10);
        // D s_100_12: cast reint s_100_11 -> u8
        let s_100_12: u8 = (s_100_11.value() as u8);
        // D s_100_13: cast zx s_100_12 -> bv
        let s_100_13: Bits = Bits::new(s_100_12 as u128, 2u16);
        // C s_100_14: const #3u : u8
        let s_100_14: u8 = 3;
        // C s_100_15: cast zx s_100_14 -> bv
        let s_100_15: Bits = Bits::new(s_100_14 as u128, 2u16);
        // D s_100_16: cmp-eq s_100_13 s_100_15
        let s_100_16: bool = ((s_100_13) == (s_100_15));
        // N s_100_17: branch s_100_16 b118 b101
        if s_100_16 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#19351 <= s_101_0
        fn_state.gs_19351 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_102_0: read-var gs#19351:u8
        let s_102_0: bool = fn_state.gs_19351;
        // N s_102_1: branch s_102_0 b117 b103
        if s_102_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#19354 <= s_103_0
        fn_state.gs_19354 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_104_0: read-var gs#19354:u8
        let s_104_0: bool = fn_state.gs_19354;
        // N s_104_1: branch s_104_0 b116 b105
        if s_104_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#19355 <= s_105_0
        fn_state.gs_19355 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_106_0: read-var gs#19355:u8
        let s_106_0: bool = fn_state.gs_19355;
        // N s_106_1: branch s_106_0 b109 b107
        if s_106_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#19358 <= s_107_0
        fn_state.gs_19358 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_108_0: read-var gs#19358:u8
        let s_108_0: bool = fn_state.gs_19358;
        // D s_108_1: not s_108_0
        let s_108_1: bool = !s_108_0;
        // D s_108_2: write-var gs#19359 <= s_108_1
        fn_state.gs_19359 = s_108_1;
        // N s_108_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_109_0: read-var accdesc.1:struct
        let s_109_0: u32 = fn_state.accdesc._1;
        // C s_109_1: const #8u : u32
        let s_109_1: u32 = 8;
        // D s_109_2: cmp-eq s_109_0 s_109_1
        let s_109_2: bool = ((s_109_0) == (s_109_1));
        // N s_109_3: branch s_109_2 b115 b110
        if s_109_2 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_110_0: read-var accdesc.1:struct
        let s_110_0: u32 = fn_state.accdesc._1;
        // C s_110_1: const #5u : u32
        let s_110_1: u32 = 5;
        // D s_110_2: cmp-eq s_110_0 s_110_1
        let s_110_2: bool = ((s_110_0) == (s_110_1));
        // N s_110_3: branch s_110_2 b114 b111
        if s_110_2 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_111_0: read-var accdesc.1:struct
        let s_111_0: u32 = fn_state.accdesc._1;
        // C s_111_1: const #6u : u32
        let s_111_1: u32 = 6;
        // D s_111_2: cmp-eq s_111_0 s_111_1
        let s_111_2: bool = ((s_111_0) == (s_111_1));
        // D s_111_3: write-var gs#19356 <= s_111_2
        fn_state.gs_19356 = s_111_2;
        // N s_111_4: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_112_0: read-var gs#19356:u8
        let s_112_0: bool = fn_state.gs_19356;
        // D s_112_1: write-var gs#19357 <= s_112_0
        fn_state.gs_19357 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_113_0: read-var gs#19357:u8
        let s_113_0: bool = fn_state.gs_19357;
        // D s_113_1: not s_113_0
        let s_113_1: bool = !s_113_0;
        // D s_113_2: write-var gs#19358 <= s_113_1
        fn_state.gs_19358 = s_113_1;
        // N s_113_3: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#19356 <= s_114_0
        fn_state.gs_19356 = s_114_0;
        // N s_114_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_115_0: const #1u : u8
        let s_115_0: bool = true;
        // D s_115_1: write-var gs#19357 <= s_115_0
        fn_state.gs_19357 = s_115_0;
        // N s_115_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_116_0: read-var accdesc.32:struct
        let s_116_0: bool = fn_state.accdesc._32;
        // D s_116_1: write-var gs#19355 <= s_116_0
        fn_state.gs_19355 = s_116_0;
        // N s_116_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_117_0: read-var tlbrecord.4:struct
        let s_117_0: u128 = fn_state.tlbrecord._4;
        // C s_117_1: const #7s : i
        let s_117_1: i128 = 7;
        // D s_117_2: cast zx s_117_0 -> bv
        let s_117_2: Bits = Bits::new(s_117_0 as u128, 128u16);
        // C s_117_3: const #1u : u64
        let s_117_3: u64 = 1;
        // D s_117_4: bit-extract s_117_2 s_117_1 s_117_3
        let s_117_4: Bits = (Bits::new(
            ((s_117_2) >> (s_117_1)).value(),
            u16::try_from(s_117_3).unwrap(),
        ));
        // D s_117_5: cast reint s_117_4 -> u8
        let s_117_5: bool = ((s_117_4.value()) != 0);
        // C s_117_6: const #0s : i
        let s_117_6: i128 = 0;
        // C s_117_7: const #0u : u64
        let s_117_7: u64 = 0;
        // D s_117_8: cast zx s_117_5 -> u64
        let s_117_8: u64 = (s_117_5 as u64);
        // C s_117_9: const #1u : u64
        let s_117_9: u64 = 1;
        // D s_117_10: and s_117_8 s_117_9
        let s_117_10: u64 = ((s_117_8) & (s_117_9));
        // D s_117_11: cmp-eq s_117_10 s_117_9
        let s_117_11: bool = ((s_117_10) == (s_117_9));
        // D s_117_12: lsl s_117_8 s_117_6
        let s_117_12: u64 = s_117_8 << s_117_6;
        // D s_117_13: or s_117_7 s_117_12
        let s_117_13: u64 = ((s_117_7) | (s_117_12));
        // D s_117_14: cmpl s_117_12
        let s_117_14: u64 = !s_117_12;
        // D s_117_15: and s_117_7 s_117_14
        let s_117_15: u64 = ((s_117_7) & (s_117_14));
        // D s_117_16: select s_117_11 s_117_13 s_117_15
        let s_117_16: u64 = if s_117_11 { s_117_13 } else { s_117_15 };
        // D s_117_17: cast trunc s_117_16 -> u8
        let s_117_17: bool = ((s_117_16) != 0);
        // D s_117_18: cast zx s_117_17 -> bv
        let s_117_18: Bits = Bits::new(s_117_17 as u128, 1u16);
        // C s_117_19: const #0u : u8
        let s_117_19: bool = false;
        // C s_117_20: cast zx s_117_19 -> bv
        let s_117_20: Bits = Bits::new(s_117_19 as u128, 1u16);
        // D s_117_21: cmp-eq s_117_18 s_117_20
        let s_117_21: bool = ((s_117_18) == (s_117_20));
        // D s_117_22: write-var gs#19354 <= s_117_21
        fn_state.gs_19354 = s_117_21;
        // N s_117_23: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_118_0: read-var walkparams.17:struct
        let s_118_0: bool = fn_state.walkparams._17;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #1u : u8
        let s_118_2: bool = true;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // N s_118_5: branch s_118_4 b121 b119
        if s_118_4 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_119_0: read-var tlbrecord.4:struct
        let s_119_0: u128 = fn_state.tlbrecord._4;
        // C s_119_1: const #51s : i
        let s_119_1: i128 = 51;
        // D s_119_2: cast zx s_119_0 -> bv
        let s_119_2: Bits = Bits::new(s_119_0 as u128, 128u16);
        // C s_119_3: const #1u : u64
        let s_119_3: u64 = 1;
        // D s_119_4: bit-extract s_119_2 s_119_1 s_119_3
        let s_119_4: Bits = (Bits::new(
            ((s_119_2) >> (s_119_1)).value(),
            u16::try_from(s_119_3).unwrap(),
        ));
        // D s_119_5: cast reint s_119_4 -> u8
        let s_119_5: bool = ((s_119_4.value()) != 0);
        // C s_119_6: const #0s : i
        let s_119_6: i128 = 0;
        // C s_119_7: const #0u : u64
        let s_119_7: u64 = 0;
        // D s_119_8: cast zx s_119_5 -> u64
        let s_119_8: u64 = (s_119_5 as u64);
        // C s_119_9: const #1u : u64
        let s_119_9: u64 = 1;
        // D s_119_10: and s_119_8 s_119_9
        let s_119_10: u64 = ((s_119_8) & (s_119_9));
        // D s_119_11: cmp-eq s_119_10 s_119_9
        let s_119_11: bool = ((s_119_10) == (s_119_9));
        // D s_119_12: lsl s_119_8 s_119_6
        let s_119_12: u64 = s_119_8 << s_119_6;
        // D s_119_13: or s_119_7 s_119_12
        let s_119_13: u64 = ((s_119_7) | (s_119_12));
        // D s_119_14: cmpl s_119_12
        let s_119_14: u64 = !s_119_12;
        // D s_119_15: and s_119_7 s_119_14
        let s_119_15: u64 = ((s_119_7) & (s_119_14));
        // D s_119_16: select s_119_11 s_119_13 s_119_15
        let s_119_16: u64 = if s_119_11 { s_119_13 } else { s_119_15 };
        // D s_119_17: cast trunc s_119_16 -> u8
        let s_119_17: bool = ((s_119_16) != 0);
        // D s_119_18: cast zx s_119_17 -> bv
        let s_119_18: Bits = Bits::new(s_119_17 as u128, 1u16);
        // C s_119_19: const #1u : u8
        let s_119_19: bool = true;
        // C s_119_20: cast zx s_119_19 -> bv
        let s_119_20: Bits = Bits::new(s_119_19 as u128, 1u16);
        // D s_119_21: cmp-eq s_119_18 s_119_20
        let s_119_21: bool = ((s_119_18) == (s_119_20));
        // D s_119_22: write-var gs#19350 <= s_119_21
        fn_state.gs_19350 = s_119_21;
        // N s_119_23: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_120_0: read-var gs#19350:u8
        let s_120_0: bool = fn_state.gs_19350;
        // D s_120_1: write-var gs#19351 <= s_120_0
        fn_state.gs_19351 = s_120_0;
        // N s_120_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // C s_121_0: const #1u : u8
        let s_121_0: bool = true;
        // D s_121_1: write-var gs#19350 <= s_121_0
        fn_state.gs_19350 = s_121_0;
        // N s_121_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4b99944cd5e0b59d {
        // D s_122_0: read-var oa.0:struct
        let s_122_0: u64 = fn_state.oa._0;
        // D s_122_1: read-var walkparams.2:struct
        let s_122_1: bool = fn_state.walkparams._2;
        // D s_122_2: read-var walkparams.14:struct
        let s_122_2: u8 = fn_state.walkparams._14;
        // D s_122_3: read-var walkparams.26:struct
        let s_122_3: u32 = fn_state.walkparams._26;
        // D s_122_4: call AArch64_OAOutOfRange(s_122_0, s_122_1, s_122_2, s_122_3)
        let s_122_4: bool = AArch64_OAOutOfRange(
            state,
            tracer,
            s_122_0,
            s_122_1,
            s_122_2,
            s_122_3,
        );
        // D s_122_5: not s_122_4
        let s_122_5: bool = !s_122_4;
        // D s_122_6: write-var gs#19347 <= s_122_5
        fn_state.gs_19347 = s_122_5;
        // N s_122_7: jump b90
        return block_90(state, tracer, fn_state);
    }
}
