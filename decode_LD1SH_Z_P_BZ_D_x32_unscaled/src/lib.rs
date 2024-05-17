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
use execute_LD1SH_Z_P_BZ_D_x32_unscaled::*;
use CurrentVL_read::*;
use common::*;
pub fn decode_LD1SH_Z_P_BZ_D_x32_unscaled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    msz: u8,
    xs: bool,
    Zm: u8,
    U: bool,
    ff: bool,
    Pg: u8,
    Rn: u8,
    Zt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        VL: i64,
        n: i64,
        offs_unsigned: bool,
        g: i64,
        msz: u8,
        xs: bool,
        Zm: u8,
        U: bool,
        ff: bool,
        Pg: u8,
        Rn: u8,
        Zt: u8,
    }
    let fn_state = FunctionState {
        msz,
        xs,
        Zm,
        U,
        ff,
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
        // N s_0_6: branch s_0_5 b12 b1
        if s_0_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Zt:u8
        let s_1_0: u8 = fn_state.Zt;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var t <= s_1_3
        fn_state.t = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var Zm:u8
        let s_1_10: u8 = fn_state.Zm;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var m <= s_1_13
        fn_state.m = s_1_13;
        // D s_1_15: read-var Pg:u8
        let s_1_15: u8 = fn_state.Pg;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 3u16);
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (s_1_16.value() as i128);
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: write-var g <= s_1_18
        fn_state.g = s_1_18;
        // D s_1_20: read-var xs:u8
        let s_1_20: bool = fn_state.xs;
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 1u16);
        // C s_1_22: const #0u : u8
        let s_1_22: bool = false;
        // C s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 1u16);
        // D s_1_24: cmp-eq s_1_21 s_1_23
        let s_1_24: bool = ((s_1_21) == (s_1_23));
        // D s_1_25: write-var offs_unsigned <= s_1_24
        fn_state.offs_unsigned = s_1_24;
        // D s_1_26: read-var VL:i64
        let s_1_26: i64 = fn_state.VL;
        // C s_1_27: const #128s : i
        let s_1_27: i128 = 128;
        // D s_1_28: cast zx s_1_26 -> i
        let s_1_28: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_29: cmp-eq s_1_28 s_1_27
        let s_1_29: bool = ((s_1_28) == (s_1_27));
        // D s_1_30: not s_1_29
        let s_1_30: bool = !s_1_29;
        // N s_1_31: branch s_1_30 b3 b2
        if s_1_30 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #128s : i64
        let s_2_0: i64 = 128;
        // C s_2_1: const #64s : i64
        let s_2_1: i64 = 64;
        // D s_2_2: read-var g:i64
        let s_2_2: i64 = fn_state.g;
        // D s_2_3: read-var m:i64
        let s_2_3: i64 = fn_state.m;
        // C s_2_4: const #16s : i64
        let s_2_4: i64 = 16;
        // D s_2_5: read-var n:i64
        let s_2_5: i64 = fn_state.n;
        // C s_2_6: const #32s : i64
        let s_2_6: i64 = 32;
        // D s_2_7: read-var offs_unsigned:u8
        let s_2_7: bool = fn_state.offs_unsigned;
        // C s_2_8: const #0s : i64
        let s_2_8: i64 = 0;
        // D s_2_9: read-var t:i64
        let s_2_9: i64 = fn_state.t;
        // C s_2_10: const #0u : u8
        let s_2_10: bool = false;
        // D s_2_11: call execute_LD1SH_Z_P_BZ_D_x32_unscaled(s_2_0, s_2_1, s_2_2, s_2_3, s_2_4, s_2_5, s_2_6, s_2_7, s_2_8, s_2_9, s_2_10)
        let s_2_11: () = execute_LD1SH_Z_P_BZ_D_x32_unscaled(
            state,
            tracer,
            s_2_0,
            s_2_1,
            s_2_2,
            s_2_3,
            s_2_4,
            s_2_5,
            s_2_6,
            s_2_7,
            s_2_8,
            s_2_9,
            s_2_10,
        );
        // N s_2_12: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VL:i64
        let s_3_0: i64 = fn_state.VL;
        // C s_3_1: const #256s : i
        let s_3_1: i128 = 256;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_1
        let s_3_3: bool = ((s_3_2) == (s_3_1));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
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
        // C s_4_0: const #256s : i64
        let s_4_0: i64 = 256;
        // C s_4_1: const #64s : i64
        let s_4_1: i64 = 64;
        // D s_4_2: read-var g:i64
        let s_4_2: i64 = fn_state.g;
        // D s_4_3: read-var m:i64
        let s_4_3: i64 = fn_state.m;
        // C s_4_4: const #16s : i64
        let s_4_4: i64 = 16;
        // D s_4_5: read-var n:i64
        let s_4_5: i64 = fn_state.n;
        // C s_4_6: const #32s : i64
        let s_4_6: i64 = 32;
        // D s_4_7: read-var offs_unsigned:u8
        let s_4_7: bool = fn_state.offs_unsigned;
        // C s_4_8: const #0s : i64
        let s_4_8: i64 = 0;
        // D s_4_9: read-var t:i64
        let s_4_9: i64 = fn_state.t;
        // C s_4_10: const #0u : u8
        let s_4_10: bool = false;
        // D s_4_11: call execute_LD1SH_Z_P_BZ_D_x32_unscaled(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4, s_4_5, s_4_6, s_4_7, s_4_8, s_4_9, s_4_10)
        let s_4_11: () = execute_LD1SH_Z_P_BZ_D_x32_unscaled(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_7,
            s_4_8,
            s_4_9,
            s_4_10,
        );
        // N s_4_12: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VL:i64
        let s_5_0: i64 = fn_state.VL;
        // C s_5_1: const #512s : i
        let s_5_1: i128 = 512;
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
        // C s_6_0: const #512s : i64
        let s_6_0: i64 = 512;
        // C s_6_1: const #64s : i64
        let s_6_1: i64 = 64;
        // D s_6_2: read-var g:i64
        let s_6_2: i64 = fn_state.g;
        // D s_6_3: read-var m:i64
        let s_6_3: i64 = fn_state.m;
        // C s_6_4: const #16s : i64
        let s_6_4: i64 = 16;
        // D s_6_5: read-var n:i64
        let s_6_5: i64 = fn_state.n;
        // C s_6_6: const #32s : i64
        let s_6_6: i64 = 32;
        // D s_6_7: read-var offs_unsigned:u8
        let s_6_7: bool = fn_state.offs_unsigned;
        // C s_6_8: const #0s : i64
        let s_6_8: i64 = 0;
        // D s_6_9: read-var t:i64
        let s_6_9: i64 = fn_state.t;
        // C s_6_10: const #0u : u8
        let s_6_10: bool = false;
        // D s_6_11: call execute_LD1SH_Z_P_BZ_D_x32_unscaled(s_6_0, s_6_1, s_6_2, s_6_3, s_6_4, s_6_5, s_6_6, s_6_7, s_6_8, s_6_9, s_6_10)
        let s_6_11: () = execute_LD1SH_Z_P_BZ_D_x32_unscaled(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_6,
            s_6_7,
            s_6_8,
            s_6_9,
            s_6_10,
        );
        // N s_6_12: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #1024s : i
        let s_7_1: i128 = 1024;
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
        // C s_8_0: const #1024s : i64
        let s_8_0: i64 = 1024;
        // C s_8_1: const #64s : i64
        let s_8_1: i64 = 64;
        // D s_8_2: read-var g:i64
        let s_8_2: i64 = fn_state.g;
        // D s_8_3: read-var m:i64
        let s_8_3: i64 = fn_state.m;
        // C s_8_4: const #16s : i64
        let s_8_4: i64 = 16;
        // D s_8_5: read-var n:i64
        let s_8_5: i64 = fn_state.n;
        // C s_8_6: const #32s : i64
        let s_8_6: i64 = 32;
        // D s_8_7: read-var offs_unsigned:u8
        let s_8_7: bool = fn_state.offs_unsigned;
        // C s_8_8: const #0s : i64
        let s_8_8: i64 = 0;
        // D s_8_9: read-var t:i64
        let s_8_9: i64 = fn_state.t;
        // C s_8_10: const #0u : u8
        let s_8_10: bool = false;
        // D s_8_11: call execute_LD1SH_Z_P_BZ_D_x32_unscaled(s_8_0, s_8_1, s_8_2, s_8_3, s_8_4, s_8_5, s_8_6, s_8_7, s_8_8, s_8_9, s_8_10)
        let s_8_11: () = execute_LD1SH_Z_P_BZ_D_x32_unscaled(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
            s_8_3,
            s_8_4,
            s_8_5,
            s_8_6,
            s_8_7,
            s_8_8,
            s_8_9,
            s_8_10,
        );
        // N s_8_12: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #2048s : i
        let s_9_1: i128 = 2048;
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
        // C s_10_0: const #2048s : i64
        let s_10_0: i64 = 2048;
        // C s_10_1: const #64s : i64
        let s_10_1: i64 = 64;
        // D s_10_2: read-var g:i64
        let s_10_2: i64 = fn_state.g;
        // D s_10_3: read-var m:i64
        let s_10_3: i64 = fn_state.m;
        // C s_10_4: const #16s : i64
        let s_10_4: i64 = 16;
        // D s_10_5: read-var n:i64
        let s_10_5: i64 = fn_state.n;
        // C s_10_6: const #32s : i64
        let s_10_6: i64 = 32;
        // D s_10_7: read-var offs_unsigned:u8
        let s_10_7: bool = fn_state.offs_unsigned;
        // C s_10_8: const #0s : i64
        let s_10_8: i64 = 0;
        // D s_10_9: read-var t:i64
        let s_10_9: i64 = fn_state.t;
        // C s_10_10: const #0u : u8
        let s_10_10: bool = false;
        // D s_10_11: call execute_LD1SH_Z_P_BZ_D_x32_unscaled(s_10_0, s_10_1, s_10_2, s_10_3, s_10_4, s_10_5, s_10_6, s_10_7, s_10_8, s_10_9, s_10_10)
        let s_10_11: () = execute_LD1SH_Z_P_BZ_D_x32_unscaled(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_7,
            s_10_8,
            s_10_9,
            s_10_10,
        );
        // N s_10_12: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
