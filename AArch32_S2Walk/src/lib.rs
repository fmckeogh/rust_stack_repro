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
use u_get_HCR2_Type_CD::*;
use FetchDescriptor::*;
use u__UNKNOWN_TTWState::*;
use AArch32_S2StartLevel::*;
use AArch32_S2InconsistentSL::*;
use place_subrange::*;
use AArch32_S2IASize::*;
use EffectiveShareability::*;
use TGxGranuleBits::*;
use AArch32_GetS2TLBContext::*;
use AArch32_DecodeDescriptorTypeLD::*;
use CreateAccDescS2TTW::*;
use HaveExtendedExecuteNeverExt::*;
use u__id::*;
use IsZero::*;
use S2DecodeMemAttrs::*;
use ContiguousSize::*;
use NormalNCMemAttr::*;
use S2TLBLookup::*;
use WalkMemAttrs::*;
use S2TLBCache::*;
use HCR2_read::*;
use TranslationSize::*;
use common::*;
pub fn AArch32_S2Walk<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    walkparams: ProductTypeb05ce25a107f0c5e,
    s1level: SumTypebf36e919d71ba1d6,
    accdesc: ProductType9878976b5bcce9c9,
    ipa: ProductTypece7c66ccb2cab13e,
) -> ProductType201519a0f62623dc {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        levels: i128,
        startlevel: i128,
        ga_21662: ProductTypef170cab34335b70c,
        gs_453617: ProductType1f0c48777d4d25a0,
        fault: ProductType1d757adad216cdef,
        stride: i128,
        baseaddress: ProductTypeda0231e9dc169f81,
        ga_21732: ProductTypef170cab34335b70c,
        gs_28048: bool,
        gs_27994: bool,
        granulebits: i128,
        ga_21694: ProductType963c597a88a9ddbc,
        return_value: ProductType201519a0f62623dc,
        finalwalkstateshadow_519: ProductType96e7acababe246a1,
        gs_27999: bool,
        gs_28027: bool,
        walkaddress: ProductTypece7c66ccb2cab13e,
        baselsb: i128,
        indexmsb: i128,
        ga_21690: ProductTypeda0231e9dc169f81,
        iasize: i128,
        tlbrecord: ProductTypee47dd77b186df56e,
        ga_21712: ProductTypeda0231e9dc169f81,
        indexlsbshadow_516: i128,
        walkstate: ProductType96e7acababe246a1,
        ga_21667: ProductTypeda0231e9dc169f81,
        ga_21670: ProductTypeda0231e9dc169f81,
        gs_28049: bool,
        indexlsb: i128,
        tlbentry: ProductTypeeb828c17bbe5e68,
        desctype: u32,
        indexmsbshadow_517: i128,
        ga_21626: ProductTypee47dd77b186df56e,
        gs_453619: ProductTypeb4cea7287e2eb9d6,
        gs_28101: bool,
        descriptor: u64,
        walkaccess: ProductType9878976b5bcce9c9,
        indexlsbshadow_518: i128,
        fault_in: ProductType1d757adad216cdef,
        walkparams: ProductTypeb05ce25a107f0c5e,
        s1level: SumTypebf36e919d71ba1d6,
        accdesc: ProductType9878976b5bcce9c9,
        ipa: ProductTypece7c66ccb2cab13e,
    }
    let fn_state = FunctionState {
        fault_in,
        walkparams,
        s1level,
        accdesc,
        ipa,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_0_0: read-var fault_in:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_1: write-var fault <= s_0_0
        fn_state.fault = s_0_0;
        // C s_0_2: const #19088u : u32
        let s_0_2: u32 = 19088;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: bool = {
            let value = state.read_register::<bool>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // N s_0_4: branch s_0_3 b55 b1
        if s_0_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_2_0: read-var walkparams.22:struct
        let s_2_0: u8 = fn_state.walkparams._22;
        // C s_2_1: const #1s : i
        let s_2_1: i128 = 1;
        // D s_2_2: cast zx s_2_0 -> bv
        let s_2_2: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #0s : i
        let s_2_5: i128 = 0;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_1 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_1)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: bool = ((s_2_7.value()) != 0);
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // C s_2_10: const #1u : u8
        let s_2_10: bool = true;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 1u16);
        // D s_2_12: cmp-eq s_2_9 s_2_11
        let s_2_12: bool = ((s_2_9) == (s_2_11));
        // D s_2_13: not s_2_12
        let s_2_13: bool = !s_2_12;
        // N s_2_14: branch s_2_13 b54 b3
        if s_2_13 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#27994 <= s_3_0
        fn_state.gs_27994 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_4_0: read-var gs#27994:u8
        let s_4_0: bool = fn_state.gs_27994;
        // N s_4_1: branch s_4_0 b53 b5
        if s_4_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_5_0: read-var walkparams:struct
        let s_5_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_5_1: call AArch32_S2InconsistentSL(s_5_0)
        let s_5_1: bool = AArch32_S2InconsistentSL(state, tracer, s_5_0);
        // D s_5_2: write-var gs#27999 <= s_5_1
        fn_state.gs_27999 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_6_0: read-var gs#27999:u8
        let s_6_0: bool = fn_state.gs_27999;
        // N s_6_1: branch s_6_0 b52 b7
        if s_6_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_7_0: read-var walkparams.25:struct
        let s_7_0: u8 = fn_state.walkparams._25;
        // D s_7_1: call AArch32_S2IASize(s_7_0)
        let s_7_1: i128 = AArch32_S2IASize(state, tracer, s_7_0);
        // D s_7_2: write-var iasize <= s_7_1
        fn_state.iasize = s_7_1;
        // D s_7_3: read-var walkparams.22:struct
        let s_7_3: u8 = fn_state.walkparams._22;
        // D s_7_4: call AArch32_S2StartLevel(s_7_3)
        let s_7_4: i128 = AArch32_S2StartLevel(state, tracer, s_7_3);
        // D s_7_5: write-var startlevel <= s_7_4
        fn_state.startlevel = s_7_4;
        // C s_7_6: const #800u : u32
        let s_7_6: u32 = 800;
        // D s_7_7: read-reg s_7_6:i64
        let s_7_7: i64 = {
            let value = state.read_register::<i64>(s_7_6 as isize);
            tracer.read_register(s_7_6 as isize, value);
            value
        };
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: read-var startlevel:i
        let s_7_9: i128 = fn_state.startlevel;
        // D s_7_10: sub s_7_8 s_7_9
        let s_7_10: i128 = ((s_7_8) - (s_7_9));
        // D s_7_11: write-var levels <= s_7_10
        fn_state.levels = s_7_10;
        // D s_7_12: read-var walkparams.26:struct
        let s_7_12: u32 = fn_state.walkparams._26;
        // D s_7_13: call TGxGranuleBits(s_7_12)
        let s_7_13: i128 = TGxGranuleBits(state, tracer, s_7_12);
        // D s_7_14: write-var granulebits <= s_7_13
        fn_state.granulebits = s_7_13;
        // C s_7_15: const #3s : i
        let s_7_15: i128 = 3;
        // D s_7_16: read-var granulebits:i
        let s_7_16: i128 = fn_state.granulebits;
        // D s_7_17: sub s_7_16 s_7_15
        let s_7_17: i128 = ((s_7_16) - (s_7_15));
        // D s_7_18: write-var stride <= s_7_17
        fn_state.stride = s_7_17;
        // C s_7_19: const #22408u : u32
        let s_7_19: u32 = 22408;
        // D s_7_20: read-reg s_7_19:u64
        let s_7_20: u64 = {
            let value = state.read_register::<u64>(s_7_19 as isize);
            tracer.read_register(s_7_19 as isize, value);
            value
        };
        // C s_7_21: const #40s : i
        let s_7_21: i128 = 40;
        // D s_7_22: cast zx s_7_20 -> bv
        let s_7_22: Bits = Bits::new(s_7_20 as u128, 64u16);
        // C s_7_23: const #1s : i64
        let s_7_23: i64 = 1;
        // C s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // C s_7_25: const #7s : i
        let s_7_25: i128 = 7;
        // C s_7_26: add s_7_25 s_7_24
        let s_7_26: i128 = (s_7_25 + s_7_24);
        // D s_7_27: bit-extract s_7_22 s_7_21 s_7_26
        let s_7_27: Bits = (Bits::new(
            ((s_7_22) >> (s_7_21)).value(),
            u16::try_from(s_7_26).unwrap(),
        ));
        // D s_7_28: cast reint s_7_27 -> u8
        let s_7_28: u8 = (s_7_27.value() as u8);
        // D s_7_29: cast zx s_7_28 -> bv
        let s_7_29: Bits = Bits::new(s_7_28 as u128, 8u16);
        // D s_7_30: call IsZero(s_7_29)
        let s_7_30: bool = IsZero(state, tracer, s_7_29);
        // D s_7_31: not s_7_30
        let s_7_31: bool = !s_7_30;
        // N s_7_32: branch s_7_31 b51 b8
        if s_7_31 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_8_0: read-var levels:i
        let s_8_0: i128 = fn_state.levels;
        // D s_8_1: read-var stride:i
        let s_8_1: i128 = fn_state.stride;
        // D s_8_2: mul s_8_0 s_8_1
        let s_8_2: i128 = ((s_8_0) * (s_8_1));
        // D s_8_3: read-var granulebits:i
        let s_8_3: i128 = fn_state.granulebits;
        // D s_8_4: add s_8_2 s_8_3
        let s_8_4: i128 = (s_8_2 + s_8_3);
        // D s_8_5: read-var iasize:i
        let s_8_5: i128 = fn_state.iasize;
        // D s_8_6: sub s_8_5 s_8_4
        let s_8_6: i128 = ((s_8_5) - (s_8_4));
        // C s_8_7: const #3s : i
        let s_8_7: i128 = 3;
        // D s_8_8: add s_8_6 s_8_7
        let s_8_8: i128 = (s_8_6 + s_8_7);
        // D s_8_9: write-var baselsb <= s_8_8
        fn_state.baselsb = s_8_8;
        // C s_8_10: const #0u : u32
        let s_8_10: u32 = 0;
        // D s_8_11: write-var baseaddress.1 <= s_8_10
        fn_state.baseaddress._1 = s_8_10;
        // D s_8_12: read-var baselsb:i
        let s_8_12: i128 = fn_state.baselsb;
        // D s_8_13: call __id(s_8_12)
        let s_8_13: i128 = u__id(state, tracer, s_8_12);
        // C s_8_14: const #0s : i
        let s_8_14: i128 = 0;
        // D s_8_15: cmp-le s_8_14 s_8_13
        let s_8_15: bool = ((s_8_14) <= (s_8_13));
        // N s_8_16: branch s_8_15 b50 b9
        if s_8_15 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#28027 <= s_9_0
        fn_state.gs_28027 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_10_0: read-var gs#28027:u8
        let s_10_0: bool = fn_state.gs_28027;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // C s_10_2: const #22408u : u32
        let s_10_2: u32 = 22408;
        // D s_10_3: read-reg s_10_2:u64
        let s_10_3: u64 = {
            let value = state.read_register::<u64>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // C s_10_4: const #56s : i
        let s_10_4: i128 = 56;
        // C s_10_5: const #39s : i
        let s_10_5: i128 = 39;
        // D s_10_6: cast zx s_10_3 -> bv
        let s_10_6: Bits = Bits::new(s_10_3 as u128, 64u16);
        // D s_10_7: read-var baselsb:i
        let s_10_7: i128 = fn_state.baselsb;
        // D s_10_8: read-var baselsb:i
        let s_10_8: i128 = fn_state.baselsb;
        // D s_10_9: call place_subrange(s_10_4, s_10_6, s_10_5, s_10_7, s_10_8)
        let s_10_9: Bits = place_subrange(
            state,
            tracer,
            s_10_4,
            s_10_6,
            s_10_5,
            s_10_7,
            s_10_8,
        );
        // D s_10_10: cast reint s_10_9 -> u56
        let s_10_10: u64 = (s_10_9.value() as u64);
        // D s_10_11: write-var baseaddress.0 <= s_10_10
        fn_state.baseaddress._0 = s_10_10;
        // D s_10_12: read-var baseaddress:struct
        let s_10_12: ProductTypeda0231e9dc169f81 = fn_state.baseaddress;
        // D s_10_13: write-var walkstate.0 <= s_10_12
        fn_state.walkstate._0 = s_10_12;
        // D s_10_14: read-var startlevel:i
        let s_10_14: i128 = fn_state.startlevel;
        // D s_10_15: write-var walkstate.6 <= s_10_14
        fn_state.walkstate._6 = s_10_14;
        // C s_10_16: const #1u : u8
        let s_10_16: bool = true;
        // D s_10_17: write-var walkstate.5 <= s_10_16
        fn_state.walkstate._5 = s_10_16;
        // D s_10_18: read-var walkparams.20:struct
        let s_10_18: u8 = fn_state.walkparams._20;
        // D s_10_19: read-var walkparams.10:struct
        let s_10_19: u8 = fn_state.walkparams._10;
        // D s_10_20: read-var walkparams.13:struct
        let s_10_20: u8 = fn_state.walkparams._13;
        // D s_10_21: call WalkMemAttrs(s_10_18, s_10_19, s_10_20)
        let s_10_21: ProductTypef170cab34335b70c = WalkMemAttrs(
            state,
            tracer,
            s_10_18,
            s_10_19,
            s_10_20,
        );
        // D s_10_22: write-var walkstate.7 <= s_10_21
        fn_state.walkstate._7 = s_10_21;
        // C s_10_23: const #1s : i
        let s_10_23: i128 = 1;
        // D s_10_24: read-var iasize:i
        let s_10_24: i128 = fn_state.iasize;
        // D s_10_25: sub s_10_24 s_10_23
        let s_10_25: i128 = ((s_10_24) - (s_10_23));
        // D s_10_26: write-var indexmsb <= s_10_25
        fn_state.indexmsb = s_10_25;
        // D s_10_27: read-var accdesc:struct
        let s_10_27: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_10_28: call CreateAccDescS2TTW(s_10_27)
        let s_10_28: ProductType9878976b5bcce9c9 = CreateAccDescS2TTW(
            state,
            tracer,
            s_10_27,
        );
        // D s_10_29: write-var walkaccess <= s_10_28
        fn_state.walkaccess = s_10_28;
        // D s_10_30: read-var ipa.7:struct
        let s_10_30: u64 = fn_state.ipa._7;
        // D s_10_31: write-var walkaddress.7 <= s_10_30
        fn_state.walkaddress._7 = s_10_30;
        // C s_10_32: const #() : ()
        let s_10_32: () = ();
        // S s_10_33: call HCR2_read(s_10_32)
        let s_10_33: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_10_32);
        // S s_10_34: call _get_HCR2_Type_CD(s_10_33)
        let s_10_34: bool = u_get_HCR2_Type_CD(state, tracer, s_10_33);
        // S s_10_35: cast zx s_10_34 -> bv
        let s_10_35: Bits = Bits::new(s_10_34 as u128, 1u16);
        // C s_10_36: const #1u : u8
        let s_10_36: bool = true;
        // C s_10_37: cast zx s_10_36 -> bv
        let s_10_37: Bits = Bits::new(s_10_36 as u128, 1u16);
        // S s_10_38: cmp-eq s_10_35 s_10_37
        let s_10_38: bool = ((s_10_35) == (s_10_37));
        // N s_10_39: branch s_10_38 b49 b11
        if s_10_38 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_11_0: read-var walkstate.7:struct
        let s_11_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_11_1: write-var walkaddress.2 <= s_11_0
        fn_state.walkaddress._2 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_12_0: read-var walkaddress.2:struct
        let s_12_0: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // D s_12_1: call EffectiveShareability(s_12_0)
        let s_12_1: u32 = EffectiveShareability(state, tracer, s_12_0);
        // D s_12_2: write-var walkaddress.2.5 <= s_12_1
        fn_state.walkaddress._2._5 = s_12_1;
        // N s_12_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_13_0: read-var walkstate.6:struct
        let s_13_0: i128 = fn_state.walkstate._6;
        // D s_13_1: write-var fault.9 <= s_13_0
        fn_state.fault._9 = s_13_0;
        // D s_13_2: read-var walkstate.6:struct
        let s_13_2: i128 = fn_state.walkstate._6;
        // C s_13_3: const #800u : u32
        let s_13_3: u32 = 800;
        // D s_13_4: read-reg s_13_3:i64
        let s_13_4: i64 = {
            let value = state.read_register::<i64>(s_13_3 as isize);
            tracer.read_register(s_13_3 as isize, value);
            value
        };
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: sub s_13_5 s_13_2
        let s_13_6: i128 = ((s_13_5) - (s_13_2));
        // D s_13_7: read-var stride:i
        let s_13_7: i128 = fn_state.stride;
        // D s_13_8: mul s_13_6 s_13_7
        let s_13_8: i128 = ((s_13_6) * (s_13_7));
        // D s_13_9: read-var granulebits:i
        let s_13_9: i128 = fn_state.granulebits;
        // D s_13_10: add s_13_8 s_13_9
        let s_13_10: i128 = (s_13_8 + s_13_9);
        // D s_13_11: write-var indexlsb <= s_13_10
        fn_state.indexlsb = s_13_10;
        // D s_13_12: read-var indexmsb:i
        let s_13_12: i128 = fn_state.indexmsb;
        // D s_13_13: write-var indexmsbshadow#517 <= s_13_12
        fn_state.indexmsbshadow_517 = s_13_12;
        // D s_13_14: read-var indexlsb:i
        let s_13_14: i128 = fn_state.indexlsb;
        // D s_13_15: write-var indexlsbshadow#518 <= s_13_14
        fn_state.indexlsbshadow_518 = s_13_14;
        // D s_13_16: read-var indexlsbshadow#518:i
        let s_13_16: i128 = fn_state.indexlsbshadow_518;
        // D s_13_17: call __id(s_13_16)
        let s_13_17: i128 = u__id(state, tracer, s_13_16);
        // C s_13_18: const #0s : i
        let s_13_18: i128 = 0;
        // D s_13_19: cmp-le s_13_18 s_13_17
        let s_13_19: bool = ((s_13_18) <= (s_13_17));
        // N s_13_20: branch s_13_19 b45 b14
        if s_13_19 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#28049 <= s_14_0
        fn_state.gs_28049 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_15_0: read-var gs#28049:u8
        let s_15_0: bool = fn_state.gs_28049;
        // N s_15_1: assert s_15_0
        let s_15_1: () = assert!(s_15_0);
        // D s_15_2: read-var indexmsbshadow#517:i
        let s_15_2: i128 = fn_state.indexmsbshadow_517;
        // D s_15_3: call __id(s_15_2)
        let s_15_3: i128 = u__id(state, tracer, s_15_2);
        // D s_15_4: cast reint s_15_3 -> i64
        let s_15_4: i64 = (s_15_3 as i64);
        // D s_15_5: read-var indexlsbshadow#518:i
        let s_15_5: i128 = fn_state.indexlsbshadow_518;
        // D s_15_6: call __id(s_15_5)
        let s_15_6: i128 = u__id(state, tracer, s_15_5);
        // D s_15_7: cast reint s_15_6 -> i64
        let s_15_7: i64 = (s_15_6 as i64);
        // D s_15_8: cast zx s_15_4 -> i
        let s_15_8: i128 = (i128::try_from(s_15_4).unwrap());
        // D s_15_9: cast zx s_15_7 -> i
        let s_15_9: i128 = (i128::try_from(s_15_7).unwrap());
        // D s_15_10: sub s_15_8 s_15_9
        let s_15_10: i128 = ((s_15_8) - (s_15_9));
        // D s_15_11: cast reint s_15_10 -> i64
        let s_15_11: i64 = (s_15_10 as i64);
        // C s_15_12: const #1s : i
        let s_15_12: i128 = 1;
        // D s_15_13: cast zx s_15_11 -> i
        let s_15_13: i128 = (i128::try_from(s_15_11).unwrap());
        // D s_15_14: add s_15_13 s_15_12
        let s_15_14: i128 = (s_15_13 + s_15_12);
        // D s_15_15: cast reint s_15_14 -> i64
        let s_15_15: i64 = (s_15_14 as i64);
        // C s_15_16: const #3s : i
        let s_15_16: i128 = 3;
        // D s_15_17: cast zx s_15_15 -> i
        let s_15_17: i128 = (i128::try_from(s_15_15).unwrap());
        // D s_15_18: add s_15_17 s_15_16
        let s_15_18: i128 = (s_15_17 + s_15_16);
        // D s_15_19: cast reint s_15_18 -> i64
        let s_15_19: i64 = (s_15_18 as i64);
        // C s_15_20: const #40s : i
        let s_15_20: i128 = 40;
        // D s_15_21: cast zx s_15_19 -> i
        let s_15_21: i128 = (i128::try_from(s_15_19).unwrap());
        // D s_15_22: cmp-ge s_15_20 s_15_21
        let s_15_22: bool = ((s_15_20) >= (s_15_21));
        // N s_15_23: assert s_15_22
        let s_15_23: () = assert!(s_15_22);
        // D s_15_24: read-var ipa.3:struct
        let s_15_24: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_15_25: write-var ga#21712 <= s_15_24
        fn_state.ga_21712 = s_15_24;
        // D s_15_26: read-var ga#21712.0:struct
        let s_15_26: u64 = fn_state.ga_21712._0;
        // C s_15_27: const #40s : i
        let s_15_27: i128 = 40;
        // C s_15_28: const #3s : i
        let s_15_28: i128 = 3;
        // D s_15_29: cast zx s_15_26 -> bv
        let s_15_29: Bits = Bits::new(s_15_26 as u128, 56u16);
        // D s_15_30: read-var indexmsbshadow#517:i
        let s_15_30: i128 = fn_state.indexmsbshadow_517;
        // D s_15_31: read-var indexlsbshadow#518:i
        let s_15_31: i128 = fn_state.indexlsbshadow_518;
        // D s_15_32: call place_subrange(s_15_27, s_15_29, s_15_30, s_15_31, s_15_28)
        let s_15_32: Bits = place_subrange(
            state,
            tracer,
            s_15_27,
            s_15_29,
            s_15_30,
            s_15_31,
            s_15_28,
        );
        // D s_15_33: cast reint s_15_32 -> u40
        let s_15_33: u64 = (s_15_32.value() as u64);
        // D s_15_34: read-var walkstate.0:struct
        let s_15_34: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_15_35: write-var ga#21667 <= s_15_34
        fn_state.ga_21667 = s_15_34;
        // D s_15_36: read-var ga#21667.0:struct
        let s_15_36: u64 = fn_state.ga_21667._0;
        // C s_15_37: const #56s : i
        let s_15_37: i128 = 56;
        // D s_15_38: cast zx s_15_33 -> bv
        let s_15_38: Bits = Bits::new(s_15_33 as u128, 40u16);
        // D s_15_39: bits-cast zx s_15_38 -> bv length s_15_37
        let s_15_39: Bits = s_15_38.zero_extend(s_15_37);
        // D s_15_40: cast reint s_15_39 -> u56
        let s_15_40: u64 = (s_15_39.value() as u64);
        // D s_15_41: cast zx s_15_36 -> bv
        let s_15_41: Bits = Bits::new(s_15_36 as u128, 56u16);
        // D s_15_42: cast zx s_15_40 -> bv
        let s_15_42: Bits = Bits::new(s_15_40 as u128, 56u16);
        // D s_15_43: or s_15_41 s_15_42
        let s_15_43: Bits = ((s_15_41) | (s_15_42));
        // D s_15_44: cast reint s_15_43 -> u56
        let s_15_44: u64 = (s_15_43.value() as u64);
        // D s_15_45: write-var walkaddress.3.0 <= s_15_44
        fn_state.walkaddress._3._0 = s_15_44;
        // D s_15_46: read-var walkstate.0:struct
        let s_15_46: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_15_47: write-var ga#21670 <= s_15_46
        fn_state.ga_21670 = s_15_46;
        // D s_15_48: read-var ga#21670.1:struct
        let s_15_48: u32 = fn_state.ga_21670._1;
        // D s_15_49: write-var walkaddress.3.1 <= s_15_48
        fn_state.walkaddress._3._1 = s_15_48;
        // C s_15_50: const #() : ()
        let s_15_50: () = ();
        // D s_15_51: create-sum enum = 0:"s_15_50"
        let s_15_51: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_15_50);
        // C s_15_52: const #() : ()
        let s_15_52: () = ();
        // D s_15_53: create-sum enum = 0:"s_15_52"
        let s_15_53: SumType755586eec3e2b646 = SumType755586eec3e2b646::_0(s_15_52);
        // D s_15_54: read-var walkaddress.7:struct
        let s_15_54: u64 = fn_state.walkaddress._7;
        // D s_15_55: read-var ipa.3:struct
        let s_15_55: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_15_56: write-var ga#21690 <= s_15_55
        fn_state.ga_21690 = s_15_55;
        // D s_15_57: read-var ga#21690.0:struct
        let s_15_57: u64 = fn_state.ga_21690._0;
        // C s_15_58: const #64s : i
        let s_15_58: i128 = 64;
        // D s_15_59: cast zx s_15_57 -> bv
        let s_15_59: Bits = Bits::new(s_15_57 as u128, 56u16);
        // D s_15_60: bits-cast zx s_15_59 -> bv length s_15_58
        let s_15_60: Bits = s_15_59.zero_extend(s_15_58);
        // D s_15_61: cast reint s_15_60 -> u64
        let s_15_61: u64 = (s_15_60.value() as u64);
        // D s_15_62: read-var walkstate.6:struct
        let s_15_62: i128 = fn_state.walkstate._6;
        // D s_15_63: create-product struct = ["s_15_61", "s_15_62"]
        let s_15_63: ProductType963c597a88a9ddbc = ProductType963c597a88a9ddbc {
            _0: s_15_61,
            _1: s_15_62,
        };
        // D s_15_64: write-var ga#21694 <= s_15_63
        fn_state.ga_21694 = s_15_63;
        // D s_15_65: read-var ga#21694.0:struct
        let s_15_65: u64 = fn_state.ga_21694._0;
        // D s_15_66: cast zx s_15_65 -> bv
        let s_15_66: Bits = Bits::new(s_15_65 as u128, 64u16);
        // D s_15_67: write-var gs#453617.0 <= s_15_66
        fn_state.gs_453617._0 = s_15_66;
        // D s_15_68: read-var ga#21694.1:struct
        let s_15_68: i128 = fn_state.ga_21694._1;
        // D s_15_69: write-var gs#453617.1 <= s_15_68
        fn_state.gs_453617._1 = s_15_68;
        // D s_15_70: read-var gs#453617:struct
        let s_15_70: ProductType1f0c48777d4d25a0 = fn_state.gs_453617;
        // D s_15_71: create-sum enum = 1:"s_15_70"
        let s_15_71: SumType3cca557f9e907281 = SumType3cca557f9e907281::_1(s_15_70);
        // C s_15_72: const #() : ()
        let s_15_72: () = ();
        // D s_15_73: create-sum enum = 0:"s_15_72"
        let s_15_73: SumTypefc0aa8a49e605a17 = SumTypefc0aa8a49e605a17::_0(s_15_72);
        // D s_15_74: read-var walkparams:struct
        let s_15_74: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_15_75: create-sum enum = 1:"s_15_74"
        let s_15_75: SumType3436044442b382d9 = SumType3436044442b382d9::_1(s_15_74);
        // D s_15_76: read-var walkaddress.2:struct
        let s_15_76: ProductTypef170cab34335b70c = fn_state.walkaddress._2;
        // C s_15_77: const #4u : u32
        let s_15_77: u32 = 4;
        // D s_15_78: read-var s1level:enum
        let s_15_78: SumTypebf36e919d71ba1d6 = fn_state.s1level;
        // D s_15_79: create-product struct = ["s_15_53", "s_15_76", "s_15_77", "s_15_78", "s_15_73", "s_15_71", "s_15_75", "s_15_54", "s_15_51"]
        let s_15_79: ProductTypeb525737120e184b3 = ProductTypeb525737120e184b3 {
            _0: s_15_53,
            _1: s_15_76,
            _2: s_15_77,
            _3: s_15_78,
            _4: s_15_73,
            _5: s_15_71,
            _6: s_15_75,
            _7: s_15_54,
            _8: s_15_51,
        };
        // D s_15_80: read-var walkparams.4:struct
        let s_15_80: bool = fn_state.walkparams._4;
        // C s_15_81: const #64s : i64
        let s_15_81: i64 = 64;
        // D s_15_82: read-var walkaddress:struct
        let s_15_82: ProductTypece7c66ccb2cab13e = fn_state.walkaddress;
        // D s_15_83: read-var walkaccess:struct
        let s_15_83: ProductType9878976b5bcce9c9 = fn_state.walkaccess;
        // D s_15_84: read-var fault:struct
        let s_15_84: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_85: call FetchDescriptor(s_15_80, s_15_82, s_15_83, s_15_84, s_15_81, s_15_79)
        let s_15_85: ProductTypeb4cea7287e2eb9d6 = FetchDescriptor(
            state,
            tracer,
            s_15_80,
            s_15_82,
            s_15_83,
            s_15_84,
            s_15_81,
            s_15_79,
        );
        // D s_15_86: write-var gs#453619 <= s_15_85
        fn_state.gs_453619 = s_15_85;
        // D s_15_87: read-var gs#453619.0:struct
        let s_15_87: ProductType1d757adad216cdef = fn_state.gs_453619._0;
        // D s_15_88: read-var gs#453619.1:struct
        let s_15_88: Bits = fn_state.gs_453619._1;
        // D s_15_89: cast reint s_15_88 -> u64
        let s_15_89: u64 = (s_15_88.value() as u64);
        // D s_15_90: write-var fault <= s_15_87
        fn_state.fault = s_15_87;
        // D s_15_91: write-var descriptor <= s_15_89
        fn_state.descriptor = s_15_89;
        // D s_15_92: read-var fault.16:struct
        let s_15_92: u32 = fn_state.fault._16;
        // C s_15_93: const #0u : u32
        let s_15_93: u32 = 0;
        // D s_15_94: cmp-eq s_15_92 s_15_93
        let s_15_94: bool = ((s_15_92) == (s_15_93));
        // N s_15_95: branch s_15_94 b44 b16
        if s_15_94 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_16_0: read-var walkstate.6:struct
        let s_16_0: i128 = fn_state.walkstate._6;
        // D s_16_1: read-var descriptor:u64
        let s_16_1: u64 = fn_state.descriptor;
        // D s_16_2: call AArch32_DecodeDescriptorTypeLD(s_16_1, s_16_0)
        let s_16_2: u32 = AArch32_DecodeDescriptorTypeLD(state, tracer, s_16_1, s_16_0);
        // D s_16_3: write-var desctype <= s_16_2
        fn_state.desctype = s_16_2;
        // C s_16_4: const #0u : u32
        let s_16_4: u32 = 0;
        // D s_16_5: read-var desctype:u32
        let s_16_5: u32 = fn_state.desctype;
        // D s_16_6: cmp-eq s_16_4 s_16_5
        let s_16_6: bool = ((s_16_4) == (s_16_5));
        // D s_16_7: not s_16_6
        let s_16_7: bool = !s_16_6;
        // N s_16_8: branch s_16_7 b39 b17
        if s_16_7 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_17_0: const #40s : i
        let s_17_0: i128 = 40;
        // D s_17_1: read-var descriptor:u64
        let s_17_1: u64 = fn_state.descriptor;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 64u16);
        // C s_17_3: const #1s : i64
        let s_17_3: i64 = 1;
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #7s : i
        let s_17_5: i128 = 7;
        // C s_17_6: add s_17_5 s_17_4
        let s_17_6: i128 = (s_17_5 + s_17_4);
        // D s_17_7: bit-extract s_17_2 s_17_0 s_17_6
        let s_17_7: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_6).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // D s_17_9: cast zx s_17_8 -> bv
        let s_17_9: Bits = Bits::new(s_17_8 as u128, 8u16);
        // D s_17_10: call IsZero(s_17_9)
        let s_17_10: bool = IsZero(state, tracer, s_17_9);
        // D s_17_11: not s_17_10
        let s_17_11: bool = !s_17_10;
        // N s_17_12: branch s_17_11 b38 b18
        if s_17_11 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_18_0: const #56s : i
        let s_18_0: i128 = 56;
        // C s_18_1: const #39s : i
        let s_18_1: i128 = 39;
        // C s_18_2: const #12s : i
        let s_18_2: i128 = 12;
        // C s_18_3: const #12s : i
        let s_18_3: i128 = 12;
        // D s_18_4: read-var descriptor:u64
        let s_18_4: u64 = fn_state.descriptor;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 64u16);
        // D s_18_6: call place_subrange(s_18_0, s_18_5, s_18_1, s_18_2, s_18_3)
        let s_18_6: Bits = place_subrange(
            state,
            tracer,
            s_18_0,
            s_18_5,
            s_18_1,
            s_18_2,
            s_18_3,
        );
        // D s_18_7: cast reint s_18_6 -> u56
        let s_18_7: u64 = (s_18_6.value() as u64);
        // D s_18_8: write-var walkstate.0.0 <= s_18_7
        fn_state.walkstate._0._0 = s_18_7;
        // D s_18_9: read-var walkstate.6:struct
        let s_18_9: i128 = fn_state.walkstate._6;
        // C s_18_10: const #1s : i
        let s_18_10: i128 = 1;
        // D s_18_11: add s_18_9 s_18_10
        let s_18_11: i128 = (s_18_9 + s_18_10);
        // D s_18_12: write-var walkstate.6 <= s_18_11
        fn_state.walkstate._6 = s_18_11;
        // C s_18_13: const #1s : i
        let s_18_13: i128 = 1;
        // D s_18_14: read-var indexlsb:i
        let s_18_14: i128 = fn_state.indexlsb;
        // D s_18_15: sub s_18_14 s_18_13
        let s_18_15: i128 = ((s_18_14) - (s_18_13));
        // D s_18_16: write-var indexmsb <= s_18_15
        fn_state.indexmsb = s_18_15;
        // N s_18_17: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_19_0: read-var desctype:u32
        let s_19_0: u32 = fn_state.desctype;
        // C s_19_1: const #1u : u32
        let s_19_1: u32 = 1;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // N s_19_3: branch s_19_2 b20 b13
        if s_19_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_20_0: read-var indexlsb:i
        let s_20_0: i128 = fn_state.indexlsb;
        // D s_20_1: write-var indexlsbshadow#516 <= s_20_0
        fn_state.indexlsbshadow_516 = s_20_0;
        // C s_20_2: const #40s : i
        let s_20_2: i128 = 40;
        // D s_20_3: read-var descriptor:u64
        let s_20_3: u64 = fn_state.descriptor;
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 64u16);
        // C s_20_5: const #1s : i64
        let s_20_5: i64 = 1;
        // C s_20_6: cast zx s_20_5 -> i
        let s_20_6: i128 = (i128::try_from(s_20_5).unwrap());
        // C s_20_7: const #7s : i
        let s_20_7: i128 = 7;
        // C s_20_8: add s_20_7 s_20_6
        let s_20_8: i128 = (s_20_7 + s_20_6);
        // D s_20_9: bit-extract s_20_4 s_20_2 s_20_8
        let s_20_9: Bits = (Bits::new(
            ((s_20_4) >> (s_20_2)).value(),
            u16::try_from(s_20_8).unwrap(),
        ));
        // D s_20_10: cast reint s_20_9 -> u8
        let s_20_10: u8 = (s_20_9.value() as u8);
        // D s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 8u16);
        // D s_20_12: call IsZero(s_20_11)
        let s_20_12: bool = IsZero(state, tracer, s_20_11);
        // D s_20_13: not s_20_12
        let s_20_13: bool = !s_20_12;
        // N s_20_14: branch s_20_13 b37 b21
        if s_20_13 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_21_0: const #10s : i
        let s_21_0: i128 = 10;
        // D s_21_1: read-var descriptor:u64
        let s_21_1: u64 = fn_state.descriptor;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 64u16);
        // C s_21_3: const #1u : u64
        let s_21_3: u64 = 1;
        // D s_21_4: bit-extract s_21_2 s_21_0 s_21_3
        let s_21_4: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_3).unwrap(),
        ));
        // D s_21_5: cast reint s_21_4 -> u8
        let s_21_5: bool = ((s_21_4.value()) != 0);
        // C s_21_6: const #0s : i
        let s_21_6: i128 = 0;
        // C s_21_7: const #0u : u64
        let s_21_7: u64 = 0;
        // D s_21_8: cast zx s_21_5 -> u64
        let s_21_8: u64 = (s_21_5 as u64);
        // C s_21_9: const #1u : u64
        let s_21_9: u64 = 1;
        // D s_21_10: and s_21_8 s_21_9
        let s_21_10: u64 = ((s_21_8) & (s_21_9));
        // D s_21_11: cmp-eq s_21_10 s_21_9
        let s_21_11: bool = ((s_21_10) == (s_21_9));
        // D s_21_12: lsl s_21_8 s_21_6
        let s_21_12: u64 = s_21_8 << s_21_6;
        // D s_21_13: or s_21_7 s_21_12
        let s_21_13: u64 = ((s_21_7) | (s_21_12));
        // D s_21_14: cmpl s_21_12
        let s_21_14: u64 = !s_21_12;
        // D s_21_15: and s_21_7 s_21_14
        let s_21_15: u64 = ((s_21_7) & (s_21_14));
        // D s_21_16: select s_21_11 s_21_13 s_21_15
        let s_21_16: u64 = if s_21_11 { s_21_13 } else { s_21_15 };
        // D s_21_17: cast trunc s_21_16 -> u8
        let s_21_17: bool = ((s_21_16) != 0);
        // D s_21_18: cast zx s_21_17 -> bv
        let s_21_18: Bits = Bits::new(s_21_17 as u128, 1u16);
        // C s_21_19: const #0u : u8
        let s_21_19: bool = false;
        // C s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 1u16);
        // D s_21_21: cmp-eq s_21_18 s_21_20
        let s_21_21: bool = ((s_21_18) == (s_21_20));
        // N s_21_22: branch s_21_21 b36 b22
        if s_21_21 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_22_0: read-var indexlsbshadow#516:i
        let s_22_0: i128 = fn_state.indexlsbshadow_516;
        // D s_22_1: call __id(s_22_0)
        let s_22_1: i128 = u__id(state, tracer, s_22_0);
        // C s_22_2: const #0s : i
        let s_22_2: i128 = 0;
        // D s_22_3: cmp-le s_22_2 s_22_1
        let s_22_3: bool = ((s_22_2) <= (s_22_1));
        // N s_22_4: branch s_22_3 b35 b23
        if s_22_3 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#28101 <= s_23_0
        fn_state.gs_28101 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_24_0: read-var gs#28101:u8
        let s_24_0: bool = fn_state.gs_28101;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // C s_24_2: const #56s : i
        let s_24_2: i128 = 56;
        // C s_24_3: const #39s : i
        let s_24_3: i128 = 39;
        // D s_24_4: read-var descriptor:u64
        let s_24_4: u64 = fn_state.descriptor;
        // D s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 64u16);
        // D s_24_6: read-var indexlsbshadow#516:i
        let s_24_6: i128 = fn_state.indexlsbshadow_516;
        // D s_24_7: read-var indexlsbshadow#516:i
        let s_24_7: i128 = fn_state.indexlsbshadow_516;
        // D s_24_8: call place_subrange(s_24_2, s_24_5, s_24_3, s_24_6, s_24_7)
        let s_24_8: Bits = place_subrange(
            state,
            tracer,
            s_24_2,
            s_24_5,
            s_24_3,
            s_24_6,
            s_24_7,
        );
        // D s_24_9: cast reint s_24_8 -> u56
        let s_24_9: u64 = (s_24_8.value() as u64);
        // D s_24_10: write-var walkstate.0.0 <= s_24_9
        fn_state.walkstate._0._0 = s_24_9;
        // C s_24_11: const #6s : i
        let s_24_11: i128 = 6;
        // D s_24_12: read-var descriptor:u64
        let s_24_12: u64 = fn_state.descriptor;
        // D s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 64u16);
        // C s_24_14: const #1s : i64
        let s_24_14: i64 = 1;
        // C s_24_15: cast zx s_24_14 -> i
        let s_24_15: i128 = (i128::try_from(s_24_14).unwrap());
        // C s_24_16: const #1s : i
        let s_24_16: i128 = 1;
        // C s_24_17: add s_24_16 s_24_15
        let s_24_17: i128 = (s_24_16 + s_24_15);
        // D s_24_18: bit-extract s_24_13 s_24_11 s_24_17
        let s_24_18: Bits = (Bits::new(
            ((s_24_13) >> (s_24_11)).value(),
            u16::try_from(s_24_17).unwrap(),
        ));
        // D s_24_19: cast reint s_24_18 -> u8
        let s_24_19: u8 = (s_24_18.value() as u8);
        // D s_24_20: write-var walkstate.9.7 <= s_24_19
        fn_state.walkstate._9._7 = s_24_19;
        // C s_24_21: const #54s : i
        let s_24_21: i128 = 54;
        // D s_24_22: read-var descriptor:u64
        let s_24_22: u64 = fn_state.descriptor;
        // D s_24_23: cast zx s_24_22 -> bv
        let s_24_23: Bits = Bits::new(s_24_22 as u128, 64u16);
        // C s_24_24: const #1u : u64
        let s_24_24: u64 = 1;
        // D s_24_25: bit-extract s_24_23 s_24_21 s_24_24
        let s_24_25: Bits = (Bits::new(
            ((s_24_23) >> (s_24_21)).value(),
            u16::try_from(s_24_24).unwrap(),
        ));
        // D s_24_26: cast reint s_24_25 -> u8
        let s_24_26: bool = ((s_24_25.value()) != 0);
        // C s_24_27: const #0s : i
        let s_24_27: i128 = 0;
        // C s_24_28: const #0u : u64
        let s_24_28: u64 = 0;
        // D s_24_29: cast zx s_24_26 -> u64
        let s_24_29: u64 = (s_24_26 as u64);
        // C s_24_30: const #1u : u64
        let s_24_30: u64 = 1;
        // D s_24_31: and s_24_29 s_24_30
        let s_24_31: u64 = ((s_24_29) & (s_24_30));
        // D s_24_32: cmp-eq s_24_31 s_24_30
        let s_24_32: bool = ((s_24_31) == (s_24_30));
        // D s_24_33: lsl s_24_29 s_24_27
        let s_24_33: u64 = s_24_29 << s_24_27;
        // D s_24_34: or s_24_28 s_24_33
        let s_24_34: u64 = ((s_24_28) | (s_24_33));
        // D s_24_35: cmpl s_24_33
        let s_24_35: u64 = !s_24_33;
        // D s_24_36: and s_24_28 s_24_35
        let s_24_36: u64 = ((s_24_28) & (s_24_35));
        // D s_24_37: select s_24_32 s_24_34 s_24_36
        let s_24_37: u64 = if s_24_32 { s_24_34 } else { s_24_36 };
        // D s_24_38: cast trunc s_24_37 -> u8
        let s_24_38: bool = ((s_24_37) != 0);
        // D s_24_39: write-var walkstate.9.12 <= s_24_38
        fn_state.walkstate._9._12 = s_24_38;
        // C s_24_40: const #() : ()
        let s_24_40: () = ();
        // S s_24_41: call HaveExtendedExecuteNeverExt(s_24_40)
        let s_24_41: bool = HaveExtendedExecuteNeverExt(state, tracer, s_24_40);
        // N s_24_42: branch s_24_41 b34 b25
        if s_24_41 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var walkstate.9.13 <= s_25_0
        fn_state.walkstate._9._13 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_26_0: const #2s : i
        let s_26_0: i128 = 2;
        // D s_26_1: read-var descriptor:u64
        let s_26_1: u64 = fn_state.descriptor;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 64u16);
        // C s_26_3: const #1s : i64
        let s_26_3: i64 = 1;
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #3s : i
        let s_26_5: i128 = 3;
        // C s_26_6: add s_26_5 s_26_4
        let s_26_6: i128 = (s_26_5 + s_26_4);
        // D s_26_7: bit-extract s_26_2 s_26_0 s_26_6
        let s_26_7: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_6).unwrap(),
        ));
        // D s_26_8: cast reint s_26_7 -> u8
        let s_26_8: u8 = (s_26_7.value() as u8);
        // C s_26_9: const #8s : i
        let s_26_9: i128 = 8;
        // D s_26_10: read-var descriptor:u64
        let s_26_10: u64 = fn_state.descriptor;
        // D s_26_11: cast zx s_26_10 -> bv
        let s_26_11: Bits = Bits::new(s_26_10 as u128, 64u16);
        // C s_26_12: const #1s : i64
        let s_26_12: i64 = 1;
        // C s_26_13: cast zx s_26_12 -> i
        let s_26_13: i128 = (i128::try_from(s_26_12).unwrap());
        // C s_26_14: const #1s : i
        let s_26_14: i128 = 1;
        // C s_26_15: add s_26_14 s_26_13
        let s_26_15: i128 = (s_26_14 + s_26_13);
        // D s_26_16: bit-extract s_26_11 s_26_9 s_26_15
        let s_26_16: Bits = (Bits::new(
            ((s_26_11) >> (s_26_9)).value(),
            u16::try_from(s_26_15).unwrap(),
        ));
        // D s_26_17: cast reint s_26_16 -> u8
        let s_26_17: u8 = (s_26_16.value() as u8);
        // C s_26_18: const #0u : u8
        let s_26_18: bool = false;
        // D s_26_19: call S2DecodeMemAttrs(s_26_8, s_26_17, s_26_18)
        let s_26_19: ProductTypef170cab34335b70c = S2DecodeMemAttrs(
            state,
            tracer,
            s_26_8,
            s_26_17,
            s_26_18,
        );
        // D s_26_20: write-var walkstate.7 <= s_26_19
        fn_state.walkstate._7 = s_26_19;
        // C s_26_21: const #52s : i
        let s_26_21: i128 = 52;
        // D s_26_22: read-var descriptor:u64
        let s_26_22: u64 = fn_state.descriptor;
        // D s_26_23: cast zx s_26_22 -> bv
        let s_26_23: Bits = Bits::new(s_26_22 as u128, 64u16);
        // C s_26_24: const #1u : u64
        let s_26_24: u64 = 1;
        // D s_26_25: bit-extract s_26_23 s_26_21 s_26_24
        let s_26_25: Bits = (Bits::new(
            ((s_26_23) >> (s_26_21)).value(),
            u16::try_from(s_26_24).unwrap(),
        ));
        // D s_26_26: cast reint s_26_25 -> u8
        let s_26_26: bool = ((s_26_25.value()) != 0);
        // C s_26_27: const #0s : i
        let s_26_27: i128 = 0;
        // C s_26_28: const #0u : u64
        let s_26_28: u64 = 0;
        // D s_26_29: cast zx s_26_26 -> u64
        let s_26_29: u64 = (s_26_26 as u64);
        // C s_26_30: const #1u : u64
        let s_26_30: u64 = 1;
        // D s_26_31: and s_26_29 s_26_30
        let s_26_31: u64 = ((s_26_29) & (s_26_30));
        // D s_26_32: cmp-eq s_26_31 s_26_30
        let s_26_32: bool = ((s_26_31) == (s_26_30));
        // D s_26_33: lsl s_26_29 s_26_27
        let s_26_33: u64 = s_26_29 << s_26_27;
        // D s_26_34: or s_26_28 s_26_33
        let s_26_34: u64 = ((s_26_28) | (s_26_33));
        // D s_26_35: cmpl s_26_33
        let s_26_35: u64 = !s_26_33;
        // D s_26_36: and s_26_28 s_26_35
        let s_26_36: u64 = ((s_26_28) & (s_26_35));
        // D s_26_37: select s_26_32 s_26_34 s_26_36
        let s_26_37: u64 = if s_26_32 { s_26_34 } else { s_26_36 };
        // D s_26_38: cast trunc s_26_37 -> u8
        let s_26_38: bool = ((s_26_37) != 0);
        // D s_26_39: write-var walkstate.1 <= s_26_38
        fn_state.walkstate._1 = s_26_38;
        // C s_26_40: const #19088u : u32
        let s_26_40: u32 = 19088;
        // D s_26_41: read-reg s_26_40:u8
        let s_26_41: bool = {
            let value = state.read_register::<bool>(s_26_40 as isize);
            tracer.read_register(s_26_40 as isize, value);
            value
        };
        // N s_26_42: branch s_26_41 b30 b27
        if s_26_41 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_28_0: read-var fault:struct
        let s_28_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_28_1: read-var walkstate:struct
        let s_28_1: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_28_2: create-product struct = ["s_28_0", "s_28_1"]
        let s_28_2: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_28_0,
            _1: s_28_1,
        };
        // D s_28_3: write-var return_value <= s_28_2
        fn_state.return_value = s_28_2;
        // N s_28_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_29_0: read-var return_value:struct
        let s_29_0: ProductType201519a0f62623dc = fn_state.return_value;
        // N s_29_1: return s_29_0
        return s_29_0;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_30_0: read-var walkstate.7:struct
        let s_30_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_30_1: write-var ga#21732 <= s_30_0
        fn_state.ga_21732 = s_30_0;
        // D s_30_2: read-var ga#21732.7:struct
        let s_30_2: bool = fn_state.ga_21732._7;
        // D s_30_3: write-var tlbcontext.14 <= s_30_2
        fn_state.tlbcontext._14 = s_30_2;
        // D s_30_4: read-var walkstate.6:struct
        let s_30_4: i128 = fn_state.walkstate._6;
        // D s_30_5: write-var tlbcontext.8 <= s_30_4
        fn_state.tlbcontext._8 = s_30_4;
        // D s_30_6: read-var tlbcontext:struct
        let s_30_6: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_30_7: write-var tlbrecord.1 <= s_30_6
        fn_state.tlbrecord._1 = s_30_6;
        // D s_30_8: read-var walkstate:struct
        let s_30_8: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_30_9: write-var tlbrecord.5 <= s_30_8
        fn_state.tlbrecord._5 = s_30_8;
        // D s_30_10: read-var walkparams.2:struct
        let s_30_10: bool = fn_state.walkparams._2;
        // D s_30_11: read-var walkparams.26:struct
        let s_30_11: u32 = fn_state.walkparams._26;
        // D s_30_12: read-var walkstate.6:struct
        let s_30_12: i128 = fn_state.walkstate._6;
        // D s_30_13: call TranslationSize(s_30_10, s_30_11, s_30_12)
        let s_30_13: i128 = TranslationSize(state, tracer, s_30_10, s_30_11, s_30_12);
        // D s_30_14: write-var tlbrecord.0 <= s_30_13
        fn_state.tlbrecord._0 = s_30_13;
        // D s_30_15: read-var walkstate.1:struct
        let s_30_15: bool = fn_state.walkstate._1;
        // D s_30_16: cast zx s_30_15 -> bv
        let s_30_16: Bits = Bits::new(s_30_15 as u128, 1u16);
        // C s_30_17: const #1u : u8
        let s_30_17: bool = true;
        // C s_30_18: cast zx s_30_17 -> bv
        let s_30_18: Bits = Bits::new(s_30_17 as u128, 1u16);
        // D s_30_19: cmp-eq s_30_16 s_30_18
        let s_30_19: bool = ((s_30_16) == (s_30_18));
        // N s_30_20: branch s_30_19 b33 b31
        if s_30_19 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_31_0: const #0s : i
        let s_31_0: i128 = 0;
        // D s_31_1: write-var tlbrecord.2 <= s_31_0
        fn_state.tlbrecord._2 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_32_0: read-var tlbrecord:struct
        let s_32_0: ProductTypee47dd77b186df56e = fn_state.tlbrecord;
        // D s_32_1: call S2TLBCache(s_32_0)
        let s_32_1: () = S2TLBCache(state, tracer, s_32_0);
        // N s_32_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_33_0: read-var walkparams.2:struct
        let s_33_0: bool = fn_state.walkparams._2;
        // D s_33_1: read-var walkparams.26:struct
        let s_33_1: u32 = fn_state.walkparams._26;
        // D s_33_2: read-var walkstate.6:struct
        let s_33_2: i128 = fn_state.walkstate._6;
        // D s_33_3: call ContiguousSize(s_33_0, s_33_1, s_33_2)
        let s_33_3: i128 = ContiguousSize(state, tracer, s_33_0, s_33_1, s_33_2);
        // D s_33_4: write-var tlbrecord.2 <= s_33_3
        fn_state.tlbrecord._2 = s_33_3;
        // N s_33_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_34_0: const #53s : i
        let s_34_0: i128 = 53;
        // D s_34_1: read-var descriptor:u64
        let s_34_1: u64 = fn_state.descriptor;
        // D s_34_2: cast zx s_34_1 -> bv
        let s_34_2: Bits = Bits::new(s_34_1 as u128, 64u16);
        // C s_34_3: const #1u : u64
        let s_34_3: u64 = 1;
        // D s_34_4: bit-extract s_34_2 s_34_0 s_34_3
        let s_34_4: Bits = (Bits::new(
            ((s_34_2) >> (s_34_0)).value(),
            u16::try_from(s_34_3).unwrap(),
        ));
        // D s_34_5: cast reint s_34_4 -> u8
        let s_34_5: bool = ((s_34_4.value()) != 0);
        // C s_34_6: const #0s : i
        let s_34_6: i128 = 0;
        // C s_34_7: const #0u : u64
        let s_34_7: u64 = 0;
        // D s_34_8: cast zx s_34_5 -> u64
        let s_34_8: u64 = (s_34_5 as u64);
        // C s_34_9: const #1u : u64
        let s_34_9: u64 = 1;
        // D s_34_10: and s_34_8 s_34_9
        let s_34_10: u64 = ((s_34_8) & (s_34_9));
        // D s_34_11: cmp-eq s_34_10 s_34_9
        let s_34_11: bool = ((s_34_10) == (s_34_9));
        // D s_34_12: lsl s_34_8 s_34_6
        let s_34_12: u64 = s_34_8 << s_34_6;
        // D s_34_13: or s_34_7 s_34_12
        let s_34_13: u64 = ((s_34_7) | (s_34_12));
        // D s_34_14: cmpl s_34_12
        let s_34_14: u64 = !s_34_12;
        // D s_34_15: and s_34_7 s_34_14
        let s_34_15: u64 = ((s_34_7) & (s_34_14));
        // D s_34_16: select s_34_11 s_34_13 s_34_15
        let s_34_16: u64 = if s_34_11 { s_34_13 } else { s_34_15 };
        // D s_34_17: cast trunc s_34_16 -> u8
        let s_34_17: bool = ((s_34_16) != 0);
        // D s_34_18: write-var walkstate.9.13 <= s_34_17
        fn_state.walkstate._9._13 = s_34_17;
        // N s_34_19: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_35_0: read-var indexlsbshadow#516:i
        let s_35_0: i128 = fn_state.indexlsbshadow_516;
        // D s_35_1: call __id(s_35_0)
        let s_35_1: i128 = u__id(state, tracer, s_35_0);
        // C s_35_2: const #39s : i
        let s_35_2: i128 = 39;
        // D s_35_3: cmp-le s_35_1 s_35_2
        let s_35_3: bool = ((s_35_1) <= (s_35_2));
        // D s_35_4: write-var gs#28101 <= s_35_3
        fn_state.gs_28101 = s_35_3;
        // N s_35_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_36_0: const #1u : u32
        let s_36_0: u32 = 1;
        // D s_36_1: write-var fault.16 <= s_36_0
        fn_state.fault._16 = s_36_0;
        // C s_36_2: const #() : ()
        let s_36_2: () = ();
        // S s_36_3: call __UNKNOWN_TTWState(s_36_2)
        let s_36_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_36_2,
        );
        // D s_36_4: read-var fault:struct
        let s_36_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_36_5: create-product struct = ["s_36_4", "s_36_3"]
        let s_36_5: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_36_4,
            _1: s_36_3,
        };
        // D s_36_6: write-var return_value <= s_36_5
        fn_state.return_value = s_36_5;
        // N s_36_7: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_37_0: const #7u : u32
        let s_37_0: u32 = 7;
        // D s_37_1: write-var fault.16 <= s_37_0
        fn_state.fault._16 = s_37_0;
        // C s_37_2: const #() : ()
        let s_37_2: () = ();
        // S s_37_3: call __UNKNOWN_TTWState(s_37_2)
        let s_37_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_37_2,
        );
        // D s_37_4: read-var fault:struct
        let s_37_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_37_5: create-product struct = ["s_37_4", "s_37_3"]
        let s_37_5: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_37_4,
            _1: s_37_3,
        };
        // D s_37_6: write-var return_value <= s_37_5
        fn_state.return_value = s_37_5;
        // N s_37_7: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_38_0: const #7u : u32
        let s_38_0: u32 = 7;
        // D s_38_1: write-var fault.16 <= s_38_0
        fn_state.fault._16 = s_38_0;
        // C s_38_2: const #() : ()
        let s_38_2: () = ();
        // S s_38_3: call __UNKNOWN_TTWState(s_38_2)
        let s_38_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_38_2,
        );
        // D s_38_4: read-var fault:struct
        let s_38_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_38_5: create-product struct = ["s_38_4", "s_38_3"]
        let s_38_5: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_38_4,
            _1: s_38_3,
        };
        // D s_38_6: write-var return_value <= s_38_5
        fn_state.return_value = s_38_5;
        // N s_38_7: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_39_0: const #2u : u32
        let s_39_0: u32 = 2;
        // D s_39_1: read-var desctype:u32
        let s_39_1: u32 = fn_state.desctype;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: not s_39_2
        let s_39_3: bool = !s_39_2;
        // N s_39_4: branch s_39_3 b41 b40
        if s_39_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_40_0: const #6u : u32
        let s_40_0: u32 = 6;
        // D s_40_1: write-var fault.16 <= s_40_0
        fn_state.fault._16 = s_40_0;
        // C s_40_2: const #() : ()
        let s_40_2: () = ();
        // S s_40_3: call __UNKNOWN_TTWState(s_40_2)
        let s_40_3: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_40_2,
        );
        // D s_40_4: read-var fault:struct
        let s_40_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_40_5: create-product struct = ["s_40_4", "s_40_3"]
        let s_40_5: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_40_4,
            _1: s_40_3,
        };
        // D s_40_6: write-var return_value <= s_40_5
        fn_state.return_value = s_40_5;
        // N s_40_7: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_41_0: const #1u : u32
        let s_41_0: u32 = 1;
        // D s_41_1: read-var desctype:u32
        let s_41_1: u32 = fn_state.desctype;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // D s_41_3: not s_41_2
        let s_41_3: bool = !s_41_2;
        // N s_41_4: branch s_41_3 b43 b42
        if s_41_3 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var walkstate.5 <= s_42_0
        fn_state.walkstate._5 = s_42_0;
        // N s_42_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_43_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call __UNKNOWN_TTWState(s_44_0)
        let s_44_1: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_44_0,
        );
        // D s_44_2: read-var fault:struct
        let s_44_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_44_3: create-product struct = ["s_44_2", "s_44_1"]
        let s_44_3: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_44_2,
            _1: s_44_1,
        };
        // D s_44_4: write-var return_value <= s_44_3
        fn_state.return_value = s_44_3;
        // N s_44_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_45_0: read-var indexlsbshadow#518:i
        let s_45_0: i128 = fn_state.indexlsbshadow_518;
        // D s_45_1: call __id(s_45_0)
        let s_45_1: i128 = u__id(state, tracer, s_45_0);
        // D s_45_2: read-var indexmsbshadow#517:i
        let s_45_2: i128 = fn_state.indexmsbshadow_517;
        // D s_45_3: call __id(s_45_2)
        let s_45_3: i128 = u__id(state, tracer, s_45_2);
        // D s_45_4: cmp-le s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) <= (s_45_3));
        // N s_45_5: branch s_45_4 b48 b46
        if s_45_4 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#28048 <= s_46_0
        fn_state.gs_28048 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_47_0: read-var gs#28048:u8
        let s_47_0: bool = fn_state.gs_28048;
        // D s_47_1: write-var gs#28049 <= s_47_0
        fn_state.gs_28049 = s_47_0;
        // N s_47_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_48_0: read-var indexmsbshadow#517:i
        let s_48_0: i128 = fn_state.indexmsbshadow_517;
        // D s_48_1: call __id(s_48_0)
        let s_48_1: i128 = u__id(state, tracer, s_48_0);
        // C s_48_2: const #56s : i
        let s_48_2: i128 = 56;
        // D s_48_3: cmp-lt s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) < (s_48_2));
        // D s_48_4: write-var gs#28048 <= s_48_3
        fn_state.gs_28048 = s_48_3;
        // N s_48_5: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call NormalNCMemAttr(s_49_0)
        let s_49_1: ProductTypef170cab34335b70c = NormalNCMemAttr(state, tracer, s_49_0);
        // D s_49_2: write-var walkaddress.2 <= s_49_1
        fn_state.walkaddress._2 = s_49_1;
        // D s_49_3: read-var walkstate.7:struct
        let s_49_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_49_4: write-var ga#21662 <= s_49_3
        fn_state.ga_21662 = s_49_3;
        // D s_49_5: read-var ga#21662.7:struct
        let s_49_5: bool = fn_state.ga_21662._7;
        // D s_49_6: write-var walkaddress.2.7 <= s_49_5
        fn_state.walkaddress._2._7 = s_49_5;
        // N s_49_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_50_0: read-var baselsb:i
        let s_50_0: i128 = fn_state.baselsb;
        // D s_50_1: call __id(s_50_0)
        let s_50_1: i128 = u__id(state, tracer, s_50_0);
        // C s_50_2: const #39s : i
        let s_50_2: i128 = 39;
        // D s_50_3: cmp-le s_50_1 s_50_2
        let s_50_3: bool = ((s_50_1) <= (s_50_2));
        // D s_50_4: write-var gs#28027 <= s_50_3
        fn_state.gs_28027 = s_50_3;
        // N s_50_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_51_0: const #7u : u32
        let s_51_0: u32 = 7;
        // D s_51_1: write-var fault.16 <= s_51_0
        fn_state.fault._16 = s_51_0;
        // C s_51_2: const #0s : i
        let s_51_2: i128 = 0;
        // D s_51_3: write-var fault.9 <= s_51_2
        fn_state.fault._9 = s_51_2;
        // C s_51_4: const #() : ()
        let s_51_4: () = ();
        // S s_51_5: call __UNKNOWN_TTWState(s_51_4)
        let s_51_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_51_4,
        );
        // D s_51_6: read-var fault:struct
        let s_51_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_51_7: create-product struct = ["s_51_6", "s_51_5"]
        let s_51_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_51_6,
            _1: s_51_5,
        };
        // D s_51_8: write-var return_value <= s_51_7
        fn_state.return_value = s_51_7;
        // N s_51_9: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_52_0: const #6u : u32
        let s_52_0: u32 = 6;
        // D s_52_1: write-var fault.16 <= s_52_0
        fn_state.fault._16 = s_52_0;
        // C s_52_2: const #1s : i
        let s_52_2: i128 = 1;
        // D s_52_3: write-var fault.9 <= s_52_2
        fn_state.fault._9 = s_52_2;
        // C s_52_4: const #() : ()
        let s_52_4: () = ();
        // S s_52_5: call __UNKNOWN_TTWState(s_52_4)
        let s_52_5: ProductType96e7acababe246a1 = u__UNKNOWN_TTWState(
            state,
            tracer,
            s_52_4,
        );
        // D s_52_6: read-var fault:struct
        let s_52_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_52_7: create-product struct = ["s_52_6", "s_52_5"]
        let s_52_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_52_6,
            _1: s_52_5,
        };
        // D s_52_8: write-var return_value <= s_52_7
        fn_state.return_value = s_52_7;
        // N s_52_9: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#27999 <= s_53_0
        fn_state.gs_27999 = s_53_0;
        // N s_53_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#27994 <= s_54_0
        fn_state.gs_27994 = s_54_0;
        // N s_54_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_55_0: read-var ipa.3:struct
        let s_55_0: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_55_1: call AArch32_GetS2TLBContext(s_55_0)
        let s_55_1: ProductTypec0d0fb0603850c4c = AArch32_GetS2TLBContext(
            state,
            tracer,
            s_55_0,
        );
        // D s_55_2: write-var tlbcontext <= s_55_1
        fn_state.tlbcontext = s_55_1;
        // D s_55_3: read-var tlbcontext:struct
        let s_55_3: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_55_4: call S2TLBLookup(s_55_3)
        let s_55_4: ProductTypeeb828c17bbe5e68 = S2TLBLookup(state, tracer, s_55_3);
        // D s_55_5: write-var tlbentry <= s_55_4
        fn_state.tlbentry = s_55_4;
        // D s_55_6: read-var tlbentry.1:struct
        let s_55_6: bool = fn_state.tlbentry._1;
        // N s_55_7: branch s_55_6 b57 b56
        if s_55_6 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // N s_56_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType201519a0f62623dc {
        // D s_57_0: read-var tlbentry.0:struct
        let s_57_0: ProductTypee47dd77b186df56e = fn_state.tlbentry._0;
        // D s_57_1: write-var ga#21626 <= s_57_0
        fn_state.ga_21626 = s_57_0;
        // D s_57_2: read-var ga#21626.5:struct
        let s_57_2: ProductType96e7acababe246a1 = fn_state.ga_21626._5;
        // D s_57_3: write-var finalwalkstateshadow#519 <= s_57_2
        fn_state.finalwalkstateshadow_519 = s_57_2;
        // D s_57_4: read-var finalwalkstateshadow#519.6:struct
        let s_57_4: i128 = fn_state.finalwalkstateshadow_519._6;
        // D s_57_5: write-var fault.9 <= s_57_4
        fn_state.fault._9 = s_57_4;
        // D s_57_6: read-var fault:struct
        let s_57_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_57_7: create-product struct = ["s_57_6", "s_57_2"]
        let s_57_7: ProductType201519a0f62623dc = ProductType201519a0f62623dc {
            _0: s_57_6,
            _1: s_57_2,
        };
        // D s_57_8: write-var return_value <= s_57_7
        fn_state.return_value = s_57_7;
        // N s_57_9: jump b29
        return block_29(state, tracer, fn_state);
    }
}
