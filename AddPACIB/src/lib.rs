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
use u_get_HCR_EL2_Type_TGE::*;
use u_get_SCTLR_EL2_Type_EnIB::*;
use AddPAC::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_API::*;
use u_get_SCR_EL3_Type_API::*;
use u_get_SCTLR_EL1_Type_EnIB::*;
use u_get_SCTLR_EL3_Type_EnIB::*;
use S1TranslationRegime__1::*;
use EL2Enabled::*;
use EL3SDDUndef::*;
use TrapPACUse::*;
use EL3SDDUndefPriority::*;
use common::*;
pub fn AddPACIB<T: Tracer>(state: &mut State, tracer: &T, x: u64, y: u64) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        gs_15216: bool,
        return_value: u64,
        gs_15220: bool,
        Enable: bool,
        gs_15227: bool,
        gs_15211: bool,
        ga_11267: u64,
        gs_15206: bool,
        TrapEL3: bool,
        gs_15207: bool,
        ga_11271: u64,
        TrapEL2: bool,
        ga_11247: u8,
        gs_15208: bool,
        gs_15213: bool,
        APIBKey_EL1: u128,
        x: u64,
        y: u64,
    }
    let fn_state = FunctionState {
        x,
        y,
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
        // C s_0_1: const #100920u : u32
        let s_0_1: u32 = 100920;
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
        // C s_0_11: const #11096u : u32
        let s_0_11: u32 = 11096;
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
        // D s_0_31: write-var APIBKey_EL1 <= s_0_30
        fn_state.APIBKey_EL1 = s_0_30;
        // C s_0_32: const #16975u : u32
        let s_0_32: u32 = 16975;
        // D s_0_33: read-reg s_0_32:u8
        let s_0_33: u8 = {
            let value = state.read_register::<u8>(s_0_32 as isize);
            tracer.read_register(s_0_32 as isize, value);
            value
        };
        // D s_0_34: write-var ga#11247 <= s_0_33
        fn_state.ga_11247 = s_0_33;
        // D s_0_35: read-var ga#11247:u8
        let s_0_35: u8 = fn_state.ga_11247;
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
        // N s_0_42: branch s_0_41 b35 b1
        if s_0_41 {
            return block_35(state, tracer, fn_state);
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
        // N s_1_7: branch s_1_6 b34 b2
        if s_1_6 {
            return block_34(state, tracer, fn_state);
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
        // D s_2_2: call _get_SCTLR_EL2_Type_EnIB(s_2_1)
        let s_2_2: bool = u_get_SCTLR_EL2_Type_EnIB(state, tracer, s_2_1);
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
        // N s_3_2: branch s_3_1 b33 b4
        if s_3_1 {
            return block_33(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#15206 <= s_4_0
        fn_state.gs_15206 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_5_0: read-var gs#15206:u8
        let s_5_0: bool = fn_state.gs_15206;
        // N s_5_1: branch s_5_0 b29 b6
        if s_5_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#15208 <= s_6_0
        fn_state.gs_15208 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var gs#15208:u8
        let s_7_0: bool = fn_state.gs_15208;
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
        // N s_7_6: branch s_7_5 b28 b8
        if s_7_5 {
            return block_28(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#15211 <= s_8_0
        fn_state.gs_15211 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var gs#15211:u8
        let s_9_0: bool = fn_state.gs_15211;
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
        // N s_10_5: branch s_10_4 b27 b11
        if s_10_4 {
            return block_27(state, tracer, fn_state);
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
        // N s_11_1: branch s_11_0 b26 b12
        if s_11_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#15227 <= s_12_0
        fn_state.gs_15227 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var gs#15227:u8
        let s_13_0: bool = fn_state.gs_15227;
        // N s_13_1: branch s_13_0 b25 b14
        if s_13_0 {
            return block_25(state, tracer, fn_state);
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
        // N s_14_1: branch s_14_0 b23 b15
        if s_14_0 {
            return block_23(state, tracer, fn_state);
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
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_16_2: read-var APIBKey_EL1:u128
        let s_16_2: u128 = fn_state.APIBKey_EL1;
        // C s_16_3: const #0u : u8
        let s_16_3: bool = false;
        // D s_16_4: call AddPAC(s_16_0, s_16_1, s_16_2, s_16_3)
        let s_16_4: u64 = AddPAC(state, tracer, s_16_0, s_16_1, s_16_2, s_16_3);
        // D s_16_5: write-var return_value <= s_16_4
        fn_state.return_value = s_16_4;
        // N s_16_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var return_value:u64
        let s_17_0: u64 = fn_state.return_value;
        // N s_17_1: return s_17_0
        return s_17_0;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call EL3SDDUndef(s_18_0)
        let s_18_1: bool = EL3SDDUndef(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b21 b19
        if s_18_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_19_0: const #424u : u32
        let s_19_0: u32 = 424;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call TrapPACUse(s_19_1)
        let s_19_2: () = TrapPACUse(state, tracer, s_19_1);
        // N s_19_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_20_0: read-var ga#11271:u64
        let s_20_0: u64 = fn_state.ga_11271;
        // D s_20_1: write-var return_value <= s_20_0
        fn_state.return_value = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_22_0: read-var return_value:u64
        let s_22_0: u64 = fn_state.return_value;
        // N s_22_1: return s_22_0
        return s_22_0;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_23_0: const #432u : u32
        let s_23_0: u32 = 432;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call TrapPACUse(s_23_1)
        let s_23_2: () = TrapPACUse(state, tracer, s_23_1);
        // N s_23_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_24_0: read-var ga#11267:u64
        let s_24_0: u64 = fn_state.ga_11267;
        // D s_24_1: write-var return_value <= s_24_0
        fn_state.return_value = s_24_0;
        // N s_24_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call EL3SDDUndefPriority(s_26_0)
        let s_26_1: bool = EL3SDDUndefPriority(state, tracer, s_26_0);
        // D s_26_2: write-var gs#15227 <= s_26_1
        fn_state.gs_15227 = s_26_1;
        // N s_26_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_27_0: read-var x:u64
        let s_27_0: u64 = fn_state.x;
        // D s_27_1: write-var return_value <= s_27_0
        fn_state.return_value = s_27_0;
        // N s_27_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_28_0: const #90704u : u32
        let s_28_0: u32 = 90704;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_SCR_EL3_Type_API(s_28_1)
        let s_28_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #0u : u8
        let s_28_4: bool = false;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#15211 <= s_28_6
        fn_state.gs_15211 = s_28_6;
        // N s_28_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_29_0: const #102552u : u32
        let s_29_0: u32 = 102552;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_HCR_EL2_Type_TGE(s_29_1)
        let s_29_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #0u : u8
        let s_29_4: bool = false;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // N s_29_7: branch s_29_6 b32 b30
        if s_29_6 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
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
        // D s_30_2: call _get_HCR_EL2_Type_E2H(s_30_1)
        let s_30_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // C s_30_4: const #0u : u8
        let s_30_4: bool = false;
        // C s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 1u16);
        // D s_30_6: cmp-eq s_30_3 s_30_5
        let s_30_6: bool = ((s_30_3) == (s_30_5));
        // D s_30_7: write-var gs#15207 <= s_30_6
        fn_state.gs_15207 = s_30_6;
        // N s_30_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_31_0: read-var gs#15207:u8
        let s_31_0: bool = fn_state.gs_15207;
        // D s_31_1: write-var gs#15208 <= s_31_0
        fn_state.gs_15208 = s_31_0;
        // N s_31_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#15207 <= s_32_0
        fn_state.gs_15207 = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_33_0: const #102552u : u32
        let s_33_0: u32 = 102552;
        // D s_33_1: read-reg s_33_0:struct
        let s_33_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call _get_HCR_EL2_Type_API(s_33_1)
        let s_33_2: bool = u_get_HCR_EL2_Type_API(state, tracer, s_33_1);
        // D s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #0u : u8
        let s_33_4: bool = false;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // D s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // D s_33_7: write-var gs#15206 <= s_33_6
        fn_state.gs_15206 = s_33_6;
        // N s_33_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_34_0: const #90272u : u32
        let s_34_0: u32 = 90272;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call _get_SCTLR_EL1_Type_EnIB(s_34_1)
        let s_34_2: bool = u_get_SCTLR_EL1_Type_EnIB(state, tracer, s_34_1);
        // D s_34_3: write-var Enable <= s_34_2
        fn_state.Enable = s_34_2;
        // N s_34_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_35_0: read-var ga#11247:u8
        let s_35_0: u8 = fn_state.ga_11247;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 2u16);
        // C s_35_2: const #440u : u32
        let s_35_2: u32 = 440;
        // D s_35_3: read-reg s_35_2:u8
        let s_35_3: u8 = {
            let value = state.read_register::<u8>(s_35_2 as isize);
            tracer.read_register(s_35_2 as isize, value);
            value
        };
        // D s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 2u16);
        // D s_35_5: cmp-eq s_35_1 s_35_4
        let s_35_5: bool = ((s_35_1) == (s_35_4));
        // D s_35_6: not s_35_5
        let s_35_6: bool = !s_35_5;
        // N s_35_7: branch s_35_6 b43 b36
        if s_35_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_36_0: const #90272u : u32
        let s_36_0: u32 = 90272;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_SCTLR_EL1_Type_EnIB(s_36_1)
        let s_36_2: bool = u_get_SCTLR_EL1_Type_EnIB(state, tracer, s_36_1);
        // D s_36_3: write-var Enable <= s_36_2
        fn_state.Enable = s_36_2;
        // C s_36_4: const #() : ()
        let s_36_4: () = ();
        // S s_36_5: call EL2Enabled(s_36_4)
        let s_36_5: bool = EL2Enabled(state, tracer, s_36_4);
        // N s_36_6: branch s_36_5 b42 b37
        if s_36_5 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#15213 <= s_37_0
        fn_state.gs_15213 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_38_0: read-var gs#15213:u8
        let s_38_0: bool = fn_state.gs_15213;
        // D s_38_1: write-var TrapEL2 <= s_38_0
        fn_state.TrapEL2 = s_38_0;
        // C s_38_2: const #424u : u32
        let s_38_2: u32 = 424;
        // D s_38_3: read-reg s_38_2:u8
        let s_38_3: u8 = {
            let value = state.read_register::<u8>(s_38_2 as isize);
            tracer.read_register(s_38_2 as isize, value);
            value
        };
        // C s_38_4: const #2u : u8
        let s_38_4: u8 = 2;
        // D s_38_5: cmp-lt s_38_3 s_38_4
        let s_38_5: bool = ((s_38_3) < (s_38_4));
        // N s_38_6: branch s_38_5 b41 b39
        if s_38_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#15216 <= s_39_0
        fn_state.gs_15216 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_40_0: read-var gs#15216:u8
        let s_40_0: bool = fn_state.gs_15216;
        // D s_40_1: write-var TrapEL3 <= s_40_0
        fn_state.TrapEL3 = s_40_0;
        // N s_40_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_41_0: const #90704u : u32
        let s_41_0: u32 = 90704;
        // D s_41_1: read-reg s_41_0:struct
        let s_41_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call _get_SCR_EL3_Type_API(s_41_1)
        let s_41_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_41_1);
        // D s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #0u : u8
        let s_41_4: bool = false;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // D s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // D s_41_7: write-var gs#15216 <= s_41_6
        fn_state.gs_15216 = s_41_6;
        // N s_41_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_42_0: const #102552u : u32
        let s_42_0: u32 = 102552;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_HCR_EL2_Type_API(s_42_1)
        let s_42_2: bool = u_get_HCR_EL2_Type_API(state, tracer, s_42_1);
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #0u : u8
        let s_42_4: bool = false;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#15213 <= s_42_6
        fn_state.gs_15213 = s_42_6;
        // N s_42_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_43_0: read-var ga#11247:u8
        let s_43_0: u8 = fn_state.ga_11247;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 2u16);
        // C s_43_2: const #432u : u32
        let s_43_2: u32 = 432;
        // D s_43_3: read-reg s_43_2:u8
        let s_43_3: u8 = {
            let value = state.read_register::<u8>(s_43_2 as isize);
            tracer.read_register(s_43_2 as isize, value);
            value
        };
        // D s_43_4: cast zx s_43_3 -> bv
        let s_43_4: Bits = Bits::new(s_43_3 as u128, 2u16);
        // D s_43_5: cmp-eq s_43_1 s_43_4
        let s_43_5: bool = ((s_43_1) == (s_43_4));
        // D s_43_6: not s_43_5
        let s_43_6: bool = !s_43_5;
        // N s_43_7: branch s_43_6 b48 b44
        if s_43_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_44_0: const #20784u : u32
        let s_44_0: u32 = 20784;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_SCTLR_EL2_Type_EnIB(s_44_1)
        let s_44_2: bool = u_get_SCTLR_EL2_Type_EnIB(state, tracer, s_44_1);
        // D s_44_3: write-var Enable <= s_44_2
        fn_state.Enable = s_44_2;
        // C s_44_4: const #0u : u8
        let s_44_4: bool = false;
        // D s_44_5: write-var TrapEL2 <= s_44_4
        fn_state.TrapEL2 = s_44_4;
        // C s_44_6: const #424u : u32
        let s_44_6: u32 = 424;
        // D s_44_7: read-reg s_44_6:u8
        let s_44_7: u8 = {
            let value = state.read_register::<u8>(s_44_6 as isize);
            tracer.read_register(s_44_6 as isize, value);
            value
        };
        // C s_44_8: const #2u : u8
        let s_44_8: u8 = 2;
        // D s_44_9: cmp-lt s_44_7 s_44_8
        let s_44_9: bool = ((s_44_7) < (s_44_8));
        // N s_44_10: branch s_44_9 b47 b45
        if s_44_9 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#15220 <= s_45_0
        fn_state.gs_15220 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_46_0: read-var gs#15220:u8
        let s_46_0: bool = fn_state.gs_15220;
        // D s_46_1: write-var TrapEL3 <= s_46_0
        fn_state.TrapEL3 = s_46_0;
        // N s_46_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_47_0: const #90704u : u32
        let s_47_0: u32 = 90704;
        // D s_47_1: read-reg s_47_0:struct
        let s_47_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call _get_SCR_EL3_Type_API(s_47_1)
        let s_47_2: bool = u_get_SCR_EL3_Type_API(state, tracer, s_47_1);
        // D s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #0u : u8
        let s_47_4: bool = false;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // D s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#15220 <= s_47_6
        fn_state.gs_15220 = s_47_6;
        // N s_47_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_48_0: read-var ga#11247:u8
        let s_48_0: u8 = fn_state.ga_11247;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 2u16);
        // C s_48_2: const #424u : u32
        let s_48_2: u32 = 424;
        // D s_48_3: read-reg s_48_2:u8
        let s_48_3: u8 = {
            let value = state.read_register::<u8>(s_48_2 as isize);
            tracer.read_register(s_48_2 as isize, value);
            value
        };
        // D s_48_4: cast zx s_48_3 -> bv
        let s_48_4: Bits = Bits::new(s_48_3 as u128, 2u16);
        // D s_48_5: cmp-eq s_48_1 s_48_4
        let s_48_5: bool = ((s_48_1) == (s_48_4));
        // D s_48_6: not s_48_5
        let s_48_6: bool = !s_48_5;
        // N s_48_7: branch s_48_6 b50 b49
        if s_48_6 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_49_0: const #17072u : u32
        let s_49_0: u32 = 17072;
        // D s_49_1: read-reg s_49_0:struct
        let s_49_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call _get_SCTLR_EL3_Type_EnIB(s_49_1)
        let s_49_2: bool = u_get_SCTLR_EL3_Type_EnIB(state, tracer, s_49_1);
        // D s_49_3: write-var Enable <= s_49_2
        fn_state.Enable = s_49_2;
        // C s_49_4: const #0u : u8
        let s_49_4: bool = false;
        // D s_49_5: write-var TrapEL2 <= s_49_4
        fn_state.TrapEL2 = s_49_4;
        // C s_49_6: const #0u : u8
        let s_49_6: bool = false;
        // D s_49_7: write-var TrapEL3 <= s_49_6
        fn_state.TrapEL3 = s_49_6;
        // N s_49_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_50_0: jump b10
        return block_10(state, tracer, fn_state);
    }
}
