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
use IsInHost::*;
use u_get_SMCR_EL3_Type_LEN::*;
use ImplementedSMEVectorLength::*;
use EL2Enabled::*;
use u_get_SMCR_EL1_Type_LEN::*;
use u_get_SMCR_EL2_Type_LEN::*;
use common::*;
pub fn CurrentSVL_read<T: Tracer>(state: &mut State, tracer: &T, gs_3880: ()) -> i64 {
    #[derive(Default)]
    struct FunctionState {
        gs_3883: bool,
        gs_3882: bool,
        vl: i128,
        gs_3881: bool,
        gs_3884: bool,
        gs_3890: bool,
        gs_3891: bool,
        gs_3880: (),
    }
    let fn_state = FunctionState {
        gs_3880,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #440u : u32
        let s_0_3: u32 = 440;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b31 b1
        if s_0_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
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
        // N s_1_7: branch s_1_6 b30 b2
        if s_1_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#3881 <= s_2_0
        fn_state.gs_3881 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_3_0: read-var gs#3881:u8
        let s_3_0: bool = fn_state.gs_3881;
        // D s_3_1: write-var gs#3882 <= s_3_0
        fn_state.gs_3882 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_4_0: read-var gs#3882:u8
        let s_4_0: bool = fn_state.gs_3882;
        // N s_4_1: branch s_4_0 b29 b5
        if s_4_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_6_0: const #16975u : u32
        let s_6_0: u32 = 16975;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 2u16);
        // C s_6_3: const #432u : u32
        let s_6_3: u32 = 432;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: cmp-eq s_6_2 s_6_5
        let s_6_6: bool = ((s_6_2) == (s_6_5));
        // N s_6_7: branch s_6_6 b28 b7
        if s_6_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_7_0: const #16975u : u32
        let s_7_0: u32 = 16975;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 2u16);
        // C s_7_3: const #448u : u32
        let s_7_3: u32 = 448;
        // D s_7_4: read-reg s_7_3:u8
        let s_7_4: u8 = {
            let value = state.read_register::<u8>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 2u16);
        // D s_7_6: cmp-eq s_7_2 s_7_5
        let s_7_6: bool = ((s_7_2) == (s_7_5));
        // N s_7_7: branch s_7_6 b27 b8
        if s_7_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#3883 <= s_8_0
        fn_state.gs_3883 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_9_0: read-var gs#3883:u8
        let s_9_0: bool = fn_state.gs_3883;
        // D s_9_1: write-var gs#3884 <= s_9_0
        fn_state.gs_3884 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_10_0: read-var gs#3884:u8
        let s_10_0: bool = fn_state.gs_3884;
        // N s_10_1: branch s_10_0 b26 b11
        if s_10_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_11_0: const #16975u : u32
        let s_11_0: u32 = 16975;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 2u16);
        // C s_11_3: const #448u : u32
        let s_11_3: u32 = 448;
        // D s_11_4: read-reg s_11_3:u8
        let s_11_4: u8 = {
            let value = state.read_register::<u8>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 2u16);
        // D s_11_6: cmp-eq s_11_2 s_11_5
        let s_11_6: bool = ((s_11_2) == (s_11_5));
        // N s_11_7: branch s_11_6 b25 b12
        if s_11_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_12_0: const #16975u : u32
        let s_12_0: u32 = 16975;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 2u16);
        // C s_12_3: const #440u : u32
        let s_12_3: u32 = 440;
        // D s_12_4: read-reg s_12_3:u8
        let s_12_4: u8 = {
            let value = state.read_register::<u8>(s_12_3 as isize);
            tracer.read_register(s_12_3 as isize, value);
            value
        };
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 2u16);
        // D s_12_6: cmp-eq s_12_2 s_12_5
        let s_12_6: bool = ((s_12_2) == (s_12_5));
        // D s_12_7: write-var gs#3890 <= s_12_6
        fn_state.gs_3890 = s_12_6;
        // N s_12_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_13_0: read-var gs#3890:u8
        let s_13_0: bool = fn_state.gs_3890;
        // N s_13_1: branch s_13_0 b24 b14
        if s_13_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#3891 <= s_14_0
        fn_state.gs_3891 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_15_0: read-var gs#3891:u8
        let s_15_0: bool = fn_state.gs_3891;
        // N s_15_1: branch s_15_0 b23 b16
        if s_15_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_17_0: const #16975u : u32
        let s_17_0: u32 = 16975;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 2u16);
        // C s_17_3: const #424u : u32
        let s_17_3: u32 = 424;
        // D s_17_4: read-reg s_17_3:u8
        let s_17_4: u8 = {
            let value = state.read_register::<u8>(s_17_3 as isize);
            tracer.read_register(s_17_3 as isize, value);
            value
        };
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 2u16);
        // D s_17_6: cmp-eq s_17_2 s_17_5
        let s_17_6: bool = ((s_17_2) == (s_17_5));
        // N s_17_7: branch s_17_6 b22 b18
        if s_17_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_18_0: const #424u : u32
        let s_18_0: u32 = 424;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // D s_18_3: cmp-lt s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) < (s_18_2));
        // N s_18_4: branch s_18_3 b21 b19
        if s_18_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // D s_20_1: read-var vl:i
        let s_20_1: i128 = fn_state.vl;
        // D s_20_2: add s_20_1 s_20_0
        let s_20_2: i128 = (s_20_1 + s_20_0);
        // C s_20_3: const #128s : i
        let s_20_3: i128 = 128;
        // D s_20_4: mul s_20_2 s_20_3
        let s_20_4: i128 = ((s_20_2) * (s_20_3));
        // D s_20_5: write-var vl <= s_20_4
        fn_state.vl = s_20_4;
        // D s_20_6: read-var vl:i
        let s_20_6: i128 = fn_state.vl;
        // D s_20_7: tail-call ImplementedSMEVectorLength(s_20_6)
        return ImplementedSMEVectorLength(state, tracer, s_20_6);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_21_0: const #90960u : u32
        let s_21_0: u32 = 90960;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_SMCR_EL3_Type_LEN(s_21_1)
        let s_21_2: u8 = u_get_SMCR_EL3_Type_LEN(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 4u16);
        // D s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (s_21_3.value() as i128);
        // D s_21_5: cast reint s_21_4 -> i64
        let s_21_5: i64 = (s_21_4 as i64);
        // D s_21_6: cast zx s_21_5 -> i
        let s_21_6: i128 = (i128::try_from(s_21_5).unwrap());
        // D s_21_7: read-var vl:i
        let s_21_7: i128 = fn_state.vl;
        // D s_21_8: cmp-lt s_21_7 s_21_6
        let s_21_8: bool = ((s_21_7) < (s_21_6));
        // D s_21_9: select s_21_8 s_21_7 s_21_6
        let s_21_9: i128 = if s_21_8 { s_21_7 } else { s_21_6 };
        // D s_21_10: write-var vl <= s_21_9
        fn_state.vl = s_21_9;
        // N s_21_11: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_22_0: const #90960u : u32
        let s_22_0: u32 = 90960;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_SMCR_EL3_Type_LEN(s_22_1)
        let s_22_2: u8 = u_get_SMCR_EL3_Type_LEN(state, tracer, s_22_1);
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 4u16);
        // D s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (s_22_3.value() as i128);
        // D s_22_5: write-var vl <= s_22_4
        fn_state.vl = s_22_4;
        // N s_22_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_23_0: const #10376u : u32
        let s_23_0: u32 = 10376;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_SMCR_EL2_Type_LEN(s_23_1)
        let s_23_2: u8 = u_get_SMCR_EL2_Type_LEN(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 4u16);
        // D s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (s_23_3.value() as i128);
        // D s_23_5: cast reint s_23_4 -> i64
        let s_23_5: i64 = (s_23_4 as i64);
        // D s_23_6: cast zx s_23_5 -> i
        let s_23_6: i128 = (i128::try_from(s_23_5).unwrap());
        // D s_23_7: read-var vl:i
        let s_23_7: i128 = fn_state.vl;
        // D s_23_8: cmp-lt s_23_7 s_23_6
        let s_23_8: bool = ((s_23_7) < (s_23_6));
        // D s_23_9: select s_23_8 s_23_7 s_23_6
        let s_23_9: i128 = if s_23_8 { s_23_7 } else { s_23_6 };
        // D s_23_10: write-var vl <= s_23_9
        fn_state.vl = s_23_9;
        // N s_23_11: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EL2Enabled(s_24_0)
        let s_24_1: bool = EL2Enabled(state, tracer, s_24_0);
        // D s_24_2: write-var gs#3891 <= s_24_1
        fn_state.gs_3891 = s_24_1;
        // N s_24_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#3890 <= s_25_0
        fn_state.gs_3890 = s_25_0;
        // N s_25_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_26_0: const #10376u : u32
        let s_26_0: u32 = 10376;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_SMCR_EL2_Type_LEN(s_26_1)
        let s_26_2: u8 = u_get_SMCR_EL2_Type_LEN(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 4u16);
        // D s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (s_26_3.value() as i128);
        // D s_26_5: write-var vl <= s_26_4
        fn_state.vl = s_26_4;
        // N s_26_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call IsInHost(s_27_0)
        let s_27_1: bool = IsInHost(state, tracer, s_27_0);
        // D s_27_2: write-var gs#3883 <= s_27_1
        fn_state.gs_3883 = s_27_1;
        // N s_27_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#3884 <= s_28_0
        fn_state.gs_3884 = s_28_0;
        // N s_28_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_29_0: const #17304u : u32
        let s_29_0: u32 = 17304;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_SMCR_EL1_Type_LEN(s_29_1)
        let s_29_2: u8 = u_get_SMCR_EL1_Type_LEN(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 4u16);
        // D s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (s_29_3.value() as i128);
        // D s_29_5: write-var vl <= s_29_4
        fn_state.vl = s_29_4;
        // N s_29_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call IsInHost(s_30_0)
        let s_30_1: bool = IsInHost(state, tracer, s_30_0);
        // S s_30_2: not s_30_1
        let s_30_2: bool = !s_30_1;
        // D s_30_3: write-var gs#3881 <= s_30_2
        fn_state.gs_3881 = s_30_2;
        // N s_30_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#3882 <= s_31_0
        fn_state.gs_3882 = s_31_0;
        // N s_31_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
