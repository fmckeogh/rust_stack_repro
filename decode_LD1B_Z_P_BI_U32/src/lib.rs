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
use HaveSVE::*;
use CurrentVL_read::*;
use execute_LD1B_Z_P_BI_U32::*;
use HaveSME::*;
use common::*;
pub fn decode_LD1B_Z_P_BI_U32<T: Tracer>(
    state: &mut State,
    tracer: &T,
    dtype: u8,
    imm4: u8,
    Pg: u8,
    Rn: u8,
    Zt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        VL: i64,
        n: i64,
        gs_232822: bool,
        offset: i64,
        g: i64,
        dtype: u8,
        imm4: u8,
        Pg: u8,
        Rn: u8,
        Zt: u8,
    }
    let fn_state = FunctionState {
        dtype,
        imm4,
        Pg,
        Rn,
        Zt,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CurrentVL_read(s_0_0)
        let s_0_1: i64 = CurrentVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var VL <= s_0_1
        fn_state.VL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSVE(s_0_3)
        let s_0_4: bool = HaveSVE(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b15 b1
        if s_0_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#232822 <= s_1_0
        fn_state.gs_232822 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#232822:u8
        let s_2_0: bool = fn_state.gs_232822;
        // N s_2_1: branch s_2_0 b14 b3
        if s_2_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Zt:u8
        let s_3_0: u8 = fn_state.Zt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var t <= s_3_3
        fn_state.t = s_3_3;
        // D s_3_5: read-var Rn:u8
        let s_3_5: u8 = fn_state.Rn;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 5u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var n <= s_3_8
        fn_state.n = s_3_8;
        // D s_3_10: read-var Pg:u8
        let s_3_10: u8 = fn_state.Pg;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 3u16);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: write-var g <= s_3_13
        fn_state.g = s_3_13;
        // D s_3_15: read-var imm4:u8
        let s_3_15: u8 = fn_state.imm4;
        // D s_3_16: cast zx s_3_15 -> bv
        let s_3_16: Bits = Bits::new(s_3_15 as u128, 4u16);
        // D s_3_17: cast sx s_3_16 -> i
        let s_3_17: i128 = {
            let sign_bit = s_3_16.length() - 1;
            let mut result = s_3_16.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: write-var offset <= s_3_18
        fn_state.offset = s_3_18;
        // D s_3_20: read-var VL:i64
        let s_3_20: i64 = fn_state.VL;
        // C s_3_21: const #128s : i
        let s_3_21: i128 = 128;
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: cmp-eq s_3_22 s_3_21
        let s_3_23: bool = ((s_3_22) == (s_3_21));
        // D s_3_24: not s_3_23
        let s_3_24: bool = !s_3_23;
        // N s_3_25: branch s_3_24 b5 b4
        if s_3_24 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: read-var offset:i64
        let s_4_1: i64 = fn_state.offset;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // C s_4_3: const #32s : i64
        let s_4_3: i64 = 32;
        // D s_4_4: read-var g:i64
        let s_4_4: i64 = fn_state.g;
        // C s_4_5: const #8s : i64
        let s_4_5: i64 = 8;
        // D s_4_6: read-var n:i64
        let s_4_6: i64 = fn_state.n;
        // D s_4_7: read-var t:i64
        let s_4_7: i64 = fn_state.t;
        // C s_4_8: const #1u : u8
        let s_4_8: bool = true;
        // D s_4_9: call execute_LD1B_Z_P_BI_U32(s_4_0, s_4_3, s_4_4, s_4_5, s_4_6, s_4_2, s_4_7, s_4_8)
        let s_4_9: () = execute_LD1B_Z_P_BI_U32(
            state,
            tracer,
            s_4_0,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_2,
            s_4_7,
            s_4_8,
        );
        // N s_4_10: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VL:i64
        let s_5_0: i64 = fn_state.VL;
        // C s_5_1: const #256s : i
        let s_5_1: i128 = 256;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #256s : i64
        let s_6_0: i64 = 256;
        // D s_6_1: read-var offset:i64
        let s_6_1: i64 = fn_state.offset;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // C s_6_3: const #32s : i64
        let s_6_3: i64 = 32;
        // D s_6_4: read-var g:i64
        let s_6_4: i64 = fn_state.g;
        // C s_6_5: const #8s : i64
        let s_6_5: i64 = 8;
        // D s_6_6: read-var n:i64
        let s_6_6: i64 = fn_state.n;
        // D s_6_7: read-var t:i64
        let s_6_7: i64 = fn_state.t;
        // C s_6_8: const #1u : u8
        let s_6_8: bool = true;
        // D s_6_9: call execute_LD1B_Z_P_BI_U32(s_6_0, s_6_3, s_6_4, s_6_5, s_6_6, s_6_2, s_6_7, s_6_8)
        let s_6_9: () = execute_LD1B_Z_P_BI_U32(
            state,
            tracer,
            s_6_0,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_6,
            s_6_2,
            s_6_7,
            s_6_8,
        );
        // N s_6_10: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #512s : i
        let s_7_1: i128 = 512;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #512s : i64
        let s_8_0: i64 = 512;
        // D s_8_1: read-var offset:i64
        let s_8_1: i64 = fn_state.offset;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // C s_8_3: const #32s : i64
        let s_8_3: i64 = 32;
        // D s_8_4: read-var g:i64
        let s_8_4: i64 = fn_state.g;
        // C s_8_5: const #8s : i64
        let s_8_5: i64 = 8;
        // D s_8_6: read-var n:i64
        let s_8_6: i64 = fn_state.n;
        // D s_8_7: read-var t:i64
        let s_8_7: i64 = fn_state.t;
        // C s_8_8: const #1u : u8
        let s_8_8: bool = true;
        // D s_8_9: call execute_LD1B_Z_P_BI_U32(s_8_0, s_8_3, s_8_4, s_8_5, s_8_6, s_8_2, s_8_7, s_8_8)
        let s_8_9: () = execute_LD1B_Z_P_BI_U32(
            state,
            tracer,
            s_8_0,
            s_8_3,
            s_8_4,
            s_8_5,
            s_8_6,
            s_8_2,
            s_8_7,
            s_8_8,
        );
        // N s_8_10: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #1024s : i
        let s_9_1: i128 = 1024;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1024s : i64
        let s_10_0: i64 = 1024;
        // D s_10_1: read-var offset:i64
        let s_10_1: i64 = fn_state.offset;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // C s_10_3: const #32s : i64
        let s_10_3: i64 = 32;
        // D s_10_4: read-var g:i64
        let s_10_4: i64 = fn_state.g;
        // C s_10_5: const #8s : i64
        let s_10_5: i64 = 8;
        // D s_10_6: read-var n:i64
        let s_10_6: i64 = fn_state.n;
        // D s_10_7: read-var t:i64
        let s_10_7: i64 = fn_state.t;
        // C s_10_8: const #1u : u8
        let s_10_8: bool = true;
        // D s_10_9: call execute_LD1B_Z_P_BI_U32(s_10_0, s_10_3, s_10_4, s_10_5, s_10_6, s_10_2, s_10_7, s_10_8)
        let s_10_9: () = execute_LD1B_Z_P_BI_U32(
            state,
            tracer,
            s_10_0,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_2,
            s_10_7,
            s_10_8,
        );
        // N s_10_10: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VL:i64
        let s_11_0: i64 = fn_state.VL;
        // C s_11_1: const #2048s : i
        let s_11_1: i128 = 2048;
        // D s_11_2: cast zx s_11_0 -> i
        let s_11_2: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) == (s_11_1));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2048s : i64
        let s_12_0: i64 = 2048;
        // D s_12_1: read-var offset:i64
        let s_12_1: i64 = fn_state.offset;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: const #32s : i64
        let s_12_3: i64 = 32;
        // D s_12_4: read-var g:i64
        let s_12_4: i64 = fn_state.g;
        // C s_12_5: const #8s : i64
        let s_12_5: i64 = 8;
        // D s_12_6: read-var n:i64
        let s_12_6: i64 = fn_state.n;
        // D s_12_7: read-var t:i64
        let s_12_7: i64 = fn_state.t;
        // C s_12_8: const #1u : u8
        let s_12_8: bool = true;
        // D s_12_9: call execute_LD1B_Z_P_BI_U32(s_12_0, s_12_3, s_12_4, s_12_5, s_12_6, s_12_2, s_12_7, s_12_8)
        let s_12_9: () = execute_LD1B_Z_P_BI_U32(
            state,
            tracer,
            s_12_0,
            s_12_3,
            s_12_4,
            s_12_5,
            s_12_6,
            s_12_2,
            s_12_7,
            s_12_8,
        );
        // N s_12_10: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveSME(s_15_0)
        let s_15_1: bool = HaveSME(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#232822 <= s_15_2
        fn_state.gs_232822 = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
