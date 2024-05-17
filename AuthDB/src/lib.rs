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
use EL3SDDUndef::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_API::*;
use u_get_SCTLR_EL1_Type_EnDB::*;
use u_get_SCTLR_EL2_Type_EnDB::*;
use u_get_SCR_EL3_Type_API::*;
use u_get_SCTLR_EL3_Type_EnDB::*;
use S1TranslationRegime__1::*;
use EL2Enabled::*;
use Auth::*;
use TrapPACUse::*;
use EL3SDDUndefPriority::*;
use common::*;
pub fn AuthDB<T: Tracer>(
    state: &mut State,
    tracer: &T,
    x: u64,
    y: u64,
    is_combined: bool,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        gs_15426: bool,
        ga_11471: u64,
        gs_15433: bool,
        APDBKey_EL1: u128,
        return_value: u64,
        Enable: bool,
        gs_15447: bool,
        ga_11446: u8,
        TrapEL3: bool,
        ga_11466: u64,
        gs_15436: bool,
        TrapEL2: bool,
        ga_11470: u64,
        gs_15428: bool,
        gs_15440: bool,
        gs_15431: bool,
        gs_15427: bool,
        x: u64,
        y: u64,
        is_combined: bool,
    }
    let fn_state = FunctionState {
        x,
        y,
        is_combined,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // C s_0_1: const #14320u : u32
        let s_0_1: u32 = 14320;
        // D s_0_2: read-reg s_0_1:u64
        let s_0_2: u64 = {
            let value = state.read_register::<u64>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 64u16);
        // C s_0_4: const #1s : i64
        let s_0_4: i64 = 1;
        // C s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // C s_0_6: const #63s : i
        let s_0_6: i128 = 63;
        // C s_0_7: add s_0_6 s_0_5
        let s_0_7: i128 = (s_0_6 + s_0_5);
        // D s_0_8: bit-extract s_0_3 s_0_0 s_0_7
        let s_0_8: Bits = (Bits::new(
            ((s_0_3) >> (s_0_0)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D s_0_9: cast reint s_0_8 -> u64
        let s_0_9: u64 = (s_0_8.value() as u64);
        // C s_0_10: const #0s : i
        let s_0_10: i128 = 0;
        // C s_0_11: const #15760u : u32
        let s_0_11: u32 = 15760;
        // D s_0_12: read-reg s_0_11:u64
        let s_0_12: u64 = {
            let value = state.read_register::<u64>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 64u16);
        // C s_0_14: const #1s : i64
        let s_0_14: i64 = 1;
        // C s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // C s_0_16: const #63s : i
        let s_0_16: i128 = 63;
        // C s_0_17: add s_0_16 s_0_15
        let s_0_17: i128 = (s_0_16 + s_0_15);
        // D s_0_18: bit-extract s_0_13 s_0_10 s_0_17
        let s_0_18: Bits = (Bits::new(
            ((s_0_13) >> (s_0_10)).value(),
            u16::try_from(s_0_17).unwrap(),
        ));
        // D s_0_19: cast reint s_0_18 -> u64
        let s_0_19: u64 = (s_0_18.value() as u64);
        // D s_0_20: cast zx s_0_9 -> bv
        let s_0_20: Bits = Bits::new(s_0_9 as u128, 64u16);
        // D s_0_21: cast zx s_0_19 -> bv
        let s_0_21: Bits = Bits::new(s_0_19 as u128, 64u16);
        // D s_0_22: cast reint s_0_20 -> u128
        let s_0_22: u128 = (s_0_20.value() as u128);
        // D s_0_23: size-of s_0_20
        let s_0_23: u16 = s_0_20.length();
        // D s_0_24: cast reint s_0_21 -> u128
        let s_0_24: u128 = (s_0_21.value() as u128);
        // D s_0_25: size-of s_0_21
        let s_0_25: u16 = s_0_21.length();
        // D s_0_26: lsl s_0_22 s_0_25
        let s_0_26: u128 = s_0_22 << s_0_25;
        // D s_0_27: or s_0_26 s_0_24
        let s_0_27: u128 = ((s_0_26) | (s_0_24));
        // D s_0_28: add s_0_23 s_0_25
        let s_0_28: u16 = (s_0_23 + s_0_25);
        // D s_0_29: create-bits s_0_27 s_0_28
        let s_0_29: Bits = Bits::new(s_0_27, s_0_28);
        // D s_0_30: cast reint s_0_29 -> u128
        let s_0_30: u128 = (s_0_29.value() as u128);
        // D s_0_31: write-var APDBKey_EL1 <= s_0_30
        fn_state.APDBKey_EL1 = s_0_30;
        // C s_0_32: const #16975u : u32
        let s_0_32: u32 = 16975;
        // D s_0_33: read-reg s_0_32:u8
        let s_0_33: u8 = {
            let value = state.read_register::<u8>(s_0_32 as isize);
            tracer.read_register(s_0_32 as isize, value);
            value
        };
        // D s_0_34: write-var ga#11446 <= s_0_33
        fn_state.ga_11446 = s_0_33;
        // D s_0_35: read-var ga#11446:u8
        let s_0_35: u8 = fn_state.ga_11446;
        // D s_0_36: cast zx s_0_35 -> bv
        let s_0_36: Bits = Bits::new(s_0_35 as u128, 2u16);
        // C s_0_37: const #448u : u32
        let s_0_37: u32 = 448;
        // D s_0_38: read-reg s_0_37:u8
        let s_0_38: u8 = {
            let value = state.read_register::<u8>(s_0_37 as isize);
            tracer.read_register(s_0_37 as isize, value);
            value
        };
        // D s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 2u16);
        // D s_0_40: cmp-eq s_0_36 s_0_39
        let s_0_40: bool = ((s_0_36) == (s_0_39));
        // D s_0_41: not s_0_40
        let s_0_41: bool = !s_0_40;
        // N s_0_42: branch s_0_41 b36 b1
        if s_0_41 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call S1TranslationRegime__1(s_1_0)
        let s_1_1: u8 = S1TranslationRegime__1(state, tracer, s_1_0);
        // S s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b35 b2
        if s_1_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_2_0: const #20784u : u32
        let s_2_0: u32 = 20784;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_SCTLR_EL2_Type_EnDB(s_2_1)
        let s_2_2: bool = u_get_SCTLR_EL2_Type_EnDB(state, tracer, s_2_1);
        // D s_2_3: write-var Enable <= s_2_2
        fn_state.Enable = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call EL2Enabled(s_3_0)
        let s_3_1: bool = EL2Enabled(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b34 b4
        if s_3_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#15426 <= s_4_0
        fn_state.gs_15426 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var gs#15426:u8
        let s_5_0: bool = fn_state.gs_15426;
        // N s_5_1: branch s_5_0 b30 b6
        if s_5_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#15428 <= s_6_0
        fn_state.gs_15428 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var gs#15428:u8
        let s_7_0: bool = fn_state.gs_15428;
        // D s_7_1: write-var TrapEL2 <= s_7_0
        fn_state.TrapEL2 = s_7_0;
        // C s_7_2: const #424u : u32
        let s_7_2: u32 = 424;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // C s_7_4: const #2u : u8
        let s_7_4: u8 = 2;
        // D s_7_5: cmp-lt s_7_3 s_7_4
        let s_7_5: bool = ((s_7_3) < (s_7_4));
        // N s_7_6: branch s_7_5 b29 b8
        if s_7_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#15431 <= s_8_0
        fn_state.gs_15431 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var gs#15431:u8
        let s_9_0: bool = fn_state.gs_15431;
        // D s_9_1: write-var TrapEL3 <= s_9_0
        fn_state.TrapEL3 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_10_0: read-var Enable:u8
        let s_10_0: bool = fn_state.Enable;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b28 b11
        if s_10_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var TrapEL3:u8
        let s_11_0: bool = fn_state.TrapEL3;
        // N s_11_1: branch s_11_0 b27 b12
        if s_11_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#15447 <= s_12_0
        fn_state.gs_15447 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var gs#15447:u8
        let s_13_0: bool = fn_state.gs_15447;
        // N s_13_1: branch s_13_0 b26 b14
        if s_13_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_14_0: read-var TrapEL2:u8
        let s_14_0: bool = fn_state.TrapEL2;
        // N s_14_1: branch s_14_0 b24 b15
        if s_14_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_15_0: read-var TrapEL3:u8
        let s_15_0: bool = fn_state.TrapEL3;
        // N s_15_1: branch s_15_0 b19 b16
        if s_15_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_16_0: read-var x:u64
        let s_16_0: u64 = fn_state.x;
        // D s_16_1: read-var y:u64
        let s_16_1: u64 = fn_state.y;
        // D s_16_2: read-var APDBKey_EL1:u128
        let s_16_2: u128 = fn_state.APDBKey_EL1;
        // C s_16_3: const #1u : u8
        let s_16_3: bool = true;
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // D s_16_5: read-var is_combined:u8
        let s_16_5: bool = fn_state.is_combined;
        // D s_16_6: call Auth(s_16_0, s_16_1, s_16_2, s_16_3, s_16_4, s_16_5)
        let s_16_6: u64 = Auth(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
            s_16_3,
            s_16_4,
            s_16_5,
        );
        // D s_16_7: write-var ga#11471 <= s_16_6
        fn_state.ga_11471 = s_16_6;
        // N s_16_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var ga#11471:u64
        let s_17_0: u64 = fn_state.ga_11471;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_18_0: read-var return_value:u64
        let s_18_0: u64 = fn_state.return_value;
        // N s_18_1: return s_18_0
        return s_18_0;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call EL3SDDUndef(s_19_0)
        let s_19_1: bool = EL3SDDUndef(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b22 b20
        if s_19_1 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_20_0: const #424u : u32
        let s_20_0: u32 = 424;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call TrapPACUse(s_20_1)
        let s_20_2: () = TrapPACUse(state, tracer, s_20_1);
        // N s_20_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_21_0: read-var ga#11470:u64
        let s_21_0: u64 = fn_state.ga_11470;
        // D s_21_1: write-var return_value <= s_21_0
        fn_state.return_value = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_23_0: read-var return_value:u64
        let s_23_0: u64 = fn_state.return_value;
        // N s_23_1: return s_23_0
        return s_23_0;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_24_0: const #432u : u32
        let s_24_0: u32 = 432;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call TrapPACUse(s_24_1)
        let s_24_2: () = TrapPACUse(state, tracer, s_24_1);
        // N s_24_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_25_0: read-var ga#11466:u64
        let s_25_0: u64 = fn_state.ga_11466;
        // D s_25_1: write-var return_value <= s_25_0
        fn_state.return_value = s_25_0;
        // N s_25_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call EL3SDDUndefPriority(s_27_0)
        let s_27_1: bool = EL3SDDUndefPriority(state, tracer, s_27_0);
        // D s_27_2: write-var gs#15447 <= s_27_1
        fn_state.gs_15447 = s_27_1;
        // N s_27_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_28_0: read-var x:u64
        let s_28_0: u64 = fn_state.x;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_29_0: const #90704u : u32
        let s_29_0: u32 = 90704;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_SCR_EL3_Type_API(s_29_1)
        let s_29_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #0u : u8
        let s_29_4: bool = false;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var gs#15431 <= s_29_6
        fn_state.gs_15431 = s_29_6;
        // N s_29_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_30_0: const #102552u : u32
        let s_30_0: u32 = 102552;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_HCR_EL2_Type_TGE(s_30_1)
        let s_30_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // C s_30_4: const #0u : u8
        let s_30_4: bool = false;
        // C s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 1u16);
        // D s_30_6: cmp-eq s_30_3 s_30_5
        let s_30_6: bool = ((s_30_3) == (s_30_5));
        // N s_30_7: branch s_30_6 b33 b31
        if s_30_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_31_0: const #102552u : u32
        let s_31_0: u32 = 102552;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_HCR_EL2_Type_E2H(s_31_1)
        let s_31_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_31_1);
        // D s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // C s_31_4: const #0u : u8
        let s_31_4: bool = false;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 1u16);
        // D s_31_6: cmp-eq s_31_3 s_31_5
        let s_31_6: bool = ((s_31_3) == (s_31_5));
        // D s_31_7: write-var gs#15427 <= s_31_6
        fn_state.gs_15427 = s_31_6;
        // N s_31_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_32_0: read-var gs#15427:u8
        let s_32_0: bool = fn_state.gs_15427;
        // D s_32_1: write-var gs#15428 <= s_32_0
        fn_state.gs_15428 = s_32_0;
        // N s_32_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#15427 <= s_33_0
        fn_state.gs_15427 = s_33_0;
        // N s_33_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_34_0: const #102552u : u32
        let s_34_0: u32 = 102552;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call _get_HCR_EL2_Type_API(s_34_1)
        let s_34_2: bool = u_get_HCR_EL2_Type_API(state, tracer, s_34_1);
        // D s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // C s_34_4: const #0u : u8
        let s_34_4: bool = false;
        // C s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 1u16);
        // D s_34_6: cmp-eq s_34_3 s_34_5
        let s_34_6: bool = ((s_34_3) == (s_34_5));
        // D s_34_7: write-var gs#15426 <= s_34_6
        fn_state.gs_15426 = s_34_6;
        // N s_34_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_35_0: const #90272u : u32
        let s_35_0: u32 = 90272;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_SCTLR_EL1_Type_EnDB(s_35_1)
        let s_35_2: bool = u_get_SCTLR_EL1_Type_EnDB(state, tracer, s_35_1);
        // D s_35_3: write-var Enable <= s_35_2
        fn_state.Enable = s_35_2;
        // N s_35_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_36_0: read-var ga#11446:u8
        let s_36_0: u8 = fn_state.ga_11446;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 2u16);
        // C s_36_2: const #440u : u32
        let s_36_2: u32 = 440;
        // D s_36_3: read-reg s_36_2:u8
        let s_36_3: u8 = {
            let value = state.read_register::<u8>(s_36_2 as isize);
            tracer.read_register(s_36_2 as isize, value);
            value
        };
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 2u16);
        // D s_36_5: cmp-eq s_36_1 s_36_4
        let s_36_5: bool = ((s_36_1) == (s_36_4));
        // D s_36_6: not s_36_5
        let s_36_6: bool = !s_36_5;
        // N s_36_7: branch s_36_6 b44 b37
        if s_36_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_37_0: const #90272u : u32
        let s_37_0: u32 = 90272;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_SCTLR_EL1_Type_EnDB(s_37_1)
        let s_37_2: bool = u_get_SCTLR_EL1_Type_EnDB(state, tracer, s_37_1);
        // D s_37_3: write-var Enable <= s_37_2
        fn_state.Enable = s_37_2;
        // C s_37_4: const #() : ()
        let s_37_4: () = ();
        // S s_37_5: call EL2Enabled(s_37_4)
        let s_37_5: bool = EL2Enabled(state, tracer, s_37_4);
        // N s_37_6: branch s_37_5 b43 b38
        if s_37_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#15433 <= s_38_0
        fn_state.gs_15433 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_39_0: read-var gs#15433:u8
        let s_39_0: bool = fn_state.gs_15433;
        // D s_39_1: write-var TrapEL2 <= s_39_0
        fn_state.TrapEL2 = s_39_0;
        // C s_39_2: const #424u : u32
        let s_39_2: u32 = 424;
        // D s_39_3: read-reg s_39_2:u8
        let s_39_3: u8 = {
            let value = state.read_register::<u8>(s_39_2 as isize);
            tracer.read_register(s_39_2 as isize, value);
            value
        };
        // C s_39_4: const #2u : u8
        let s_39_4: u8 = 2;
        // D s_39_5: cmp-lt s_39_3 s_39_4
        let s_39_5: bool = ((s_39_3) < (s_39_4));
        // N s_39_6: branch s_39_5 b42 b40
        if s_39_5 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#15436 <= s_40_0
        fn_state.gs_15436 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_41_0: read-var gs#15436:u8
        let s_41_0: bool = fn_state.gs_15436;
        // D s_41_1: write-var TrapEL3 <= s_41_0
        fn_state.TrapEL3 = s_41_0;
        // N s_41_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_42_0: const #90704u : u32
        let s_42_0: u32 = 90704;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_SCR_EL3_Type_API(s_42_1)
        let s_42_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_42_1);
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #0u : u8
        let s_42_4: bool = false;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#15436 <= s_42_6
        fn_state.gs_15436 = s_42_6;
        // N s_42_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_43_0: const #102552u : u32
        let s_43_0: u32 = 102552;
        // D s_43_1: read-reg s_43_0:struct
        let s_43_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call _get_HCR_EL2_Type_API(s_43_1)
        let s_43_2: bool = u_get_HCR_EL2_Type_API(state, tracer, s_43_1);
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #0u : u8
        let s_43_4: bool = false;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // D s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // D s_43_7: write-var gs#15433 <= s_43_6
        fn_state.gs_15433 = s_43_6;
        // N s_43_8: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_44_0: read-var ga#11446:u8
        let s_44_0: u8 = fn_state.ga_11446;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 2u16);
        // C s_44_2: const #432u : u32
        let s_44_2: u32 = 432;
        // D s_44_3: read-reg s_44_2:u8
        let s_44_3: u8 = {
            let value = state.read_register::<u8>(s_44_2 as isize);
            tracer.read_register(s_44_2 as isize, value);
            value
        };
        // D s_44_4: cast zx s_44_3 -> bv
        let s_44_4: Bits = Bits::new(s_44_3 as u128, 2u16);
        // D s_44_5: cmp-eq s_44_1 s_44_4
        let s_44_5: bool = ((s_44_1) == (s_44_4));
        // D s_44_6: not s_44_5
        let s_44_6: bool = !s_44_5;
        // N s_44_7: branch s_44_6 b49 b45
        if s_44_6 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_45_0: const #20784u : u32
        let s_45_0: u32 = 20784;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_SCTLR_EL2_Type_EnDB(s_45_1)
        let s_45_2: bool = u_get_SCTLR_EL2_Type_EnDB(state, tracer, s_45_1);
        // D s_45_3: write-var Enable <= s_45_2
        fn_state.Enable = s_45_2;
        // C s_45_4: const #0u : u8
        let s_45_4: bool = false;
        // D s_45_5: write-var TrapEL2 <= s_45_4
        fn_state.TrapEL2 = s_45_4;
        // C s_45_6: const #424u : u32
        let s_45_6: u32 = 424;
        // D s_45_7: read-reg s_45_6:u8
        let s_45_7: u8 = {
            let value = state.read_register::<u8>(s_45_6 as isize);
            tracer.read_register(s_45_6 as isize, value);
            value
        };
        // C s_45_8: const #2u : u8
        let s_45_8: u8 = 2;
        // D s_45_9: cmp-lt s_45_7 s_45_8
        let s_45_9: bool = ((s_45_7) < (s_45_8));
        // N s_45_10: branch s_45_9 b48 b46
        if s_45_9 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#15440 <= s_46_0
        fn_state.gs_15440 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_47_0: read-var gs#15440:u8
        let s_47_0: bool = fn_state.gs_15440;
        // D s_47_1: write-var TrapEL3 <= s_47_0
        fn_state.TrapEL3 = s_47_0;
        // N s_47_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_48_0: const #90704u : u32
        let s_48_0: u32 = 90704;
        // D s_48_1: read-reg s_48_0:struct
        let s_48_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call _get_SCR_EL3_Type_API(s_48_1)
        let s_48_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_48_1);
        // D s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // C s_48_4: const #0u : u8
        let s_48_4: bool = false;
        // C s_48_5: cast zx s_48_4 -> bv
        let s_48_5: Bits = Bits::new(s_48_4 as u128, 1u16);
        // D s_48_6: cmp-eq s_48_3 s_48_5
        let s_48_6: bool = ((s_48_3) == (s_48_5));
        // D s_48_7: write-var gs#15440 <= s_48_6
        fn_state.gs_15440 = s_48_6;
        // N s_48_8: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_49_0: read-var ga#11446:u8
        let s_49_0: u8 = fn_state.ga_11446;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 2u16);
        // C s_49_2: const #424u : u32
        let s_49_2: u32 = 424;
        // D s_49_3: read-reg s_49_2:u8
        let s_49_3: u8 = {
            let value = state.read_register::<u8>(s_49_2 as isize);
            tracer.read_register(s_49_2 as isize, value);
            value
        };
        // D s_49_4: cast zx s_49_3 -> bv
        let s_49_4: Bits = Bits::new(s_49_3 as u128, 2u16);
        // D s_49_5: cmp-eq s_49_1 s_49_4
        let s_49_5: bool = ((s_49_1) == (s_49_4));
        // D s_49_6: not s_49_5
        let s_49_6: bool = !s_49_5;
        // N s_49_7: branch s_49_6 b51 b50
        if s_49_6 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_50_0: const #17072u : u32
        let s_50_0: u32 = 17072;
        // D s_50_1: read-reg s_50_0:struct
        let s_50_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call _get_SCTLR_EL3_Type_EnDB(s_50_1)
        let s_50_2: bool = u_get_SCTLR_EL3_Type_EnDB(state, tracer, s_50_1);
        // D s_50_3: write-var Enable <= s_50_2
        fn_state.Enable = s_50_2;
        // C s_50_4: const #0u : u8
        let s_50_4: bool = false;
        // D s_50_5: write-var TrapEL2 <= s_50_4
        fn_state.TrapEL2 = s_50_4;
        // C s_50_6: const #0u : u8
        let s_50_6: bool = false;
        // D s_50_7: write-var TrapEL3 <= s_50_6
        fn_state.TrapEL3 = s_50_6;
        // N s_50_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_51_0: jump b10
        return block_10(state, tracer, fn_state);
    }
}
