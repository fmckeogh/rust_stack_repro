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
use FPSCR_read::*;
use CheckVFPEnabled::*;
use FPToFixed::*;
use S_set::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VCVTA_vfp_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i64,
    rounding: u32,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_914767: Bits,
        gs_914778: Bits,
        gs_914772: Bits,
        d: i64,
        esize: i64,
        m: i64,
        rounding: u32,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        esize,
        m,
        rounding,
        is_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var esize:i64
        let s_1_0: i64 = fn_state.esize;
        // C s_1_1: const #16s : i
        let s_1_1: i128 = 16;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_1
        let s_1_3: bool = ((s_1_2) == (s_1_1));
        // D s_1_4: not s_1_3
        let s_1_4: bool = !s_1_3;
        // N s_1_5: branch s_1_4 b4 b2
        if s_1_4 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var m:i64
        let s_2_0: i64 = fn_state.m;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call S_read(s_2_1)
        let s_2_2: u32 = S_read(state, tracer, s_2_1);
        // C s_2_3: const #0s : i
        let s_2_3: i128 = 0;
        // D s_2_4: cast zx s_2_2 -> bv
        let s_2_4: Bits = Bits::new(s_2_2 as u128, 32u16);
        // C s_2_5: const #1s : i64
        let s_2_5: i64 = 1;
        // C s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_7: const #15s : i
        let s_2_7: i128 = 15;
        // C s_2_8: add s_2_7 s_2_6
        let s_2_8: i128 = (s_2_7 + s_2_6);
        // D s_2_9: bit-extract s_2_4 s_2_3 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_4) >> (s_2_3)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u16
        let s_2_10: u16 = (s_2_9.value() as u16);
        // C s_2_11: const #() : ()
        let s_2_11: () = ();
        // S s_2_12: call FPSCR_read(s_2_11)
        let s_2_12: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_2_11);
        // C s_2_13: const #32s : i64
        let s_2_13: i64 = 32;
        // C s_2_14: const #0s : i
        let s_2_14: i128 = 0;
        // D s_2_15: cast zx s_2_10 -> bv
        let s_2_15: Bits = Bits::new(s_2_10 as u128, 16u16);
        // D s_2_16: read-var is_unsigned:u8
        let s_2_16: bool = fn_state.is_unsigned;
        // D s_2_17: read-var rounding:u32
        let s_2_17: u32 = fn_state.rounding;
        // D s_2_18: call FPToFixed(s_2_15, s_2_14, s_2_16, s_2_12, s_2_17, s_2_13)
        let s_2_18: Bits = FPToFixed(
            state,
            tracer,
            s_2_15,
            s_2_14,
            s_2_16,
            s_2_12,
            s_2_17,
            s_2_13,
        );
        // D s_2_19: write-var gs#914767 <= s_2_18
        fn_state.gs_914767 = s_2_18;
        // N s_2_20: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#914767:bv
        let s_3_0: Bits = fn_state.gs_914767;
        // D s_3_1: cast reint s_3_0 -> u32
        let s_3_1: u32 = (s_3_0.value() as u32);
        // D s_3_2: read-var d:i64
        let s_3_2: i64 = fn_state.d;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: call S_set(s_3_3, s_3_1)
        let s_3_4: () = S_set(state, tracer, s_3_3, s_3_1);
        // N s_3_5: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esize:i64
        let s_4_0: i64 = fn_state.esize;
        // C s_4_1: const #32s : i
        let s_4_1: i128 = 32;
        // D s_4_2: cast zx s_4_0 -> i
        let s_4_2: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_1
        let s_4_3: bool = ((s_4_2) == (s_4_1));
        // D s_4_4: not s_4_3
        let s_4_4: bool = !s_4_3;
        // N s_4_5: branch s_4_4 b7 b5
        if s_4_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var m:i64
        let s_5_0: i64 = fn_state.m;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call S_read(s_5_1)
        let s_5_2: u32 = S_read(state, tracer, s_5_1);
        // C s_5_3: const #() : ()
        let s_5_3: () = ();
        // S s_5_4: call FPSCR_read(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_5_3);
        // C s_5_5: const #32s : i64
        let s_5_5: i64 = 32;
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // D s_5_7: cast zx s_5_2 -> bv
        let s_5_7: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_8: read-var is_unsigned:u8
        let s_5_8: bool = fn_state.is_unsigned;
        // D s_5_9: read-var rounding:u32
        let s_5_9: u32 = fn_state.rounding;
        // D s_5_10: call FPToFixed(s_5_7, s_5_6, s_5_8, s_5_4, s_5_9, s_5_5)
        let s_5_10: Bits = FPToFixed(
            state,
            tracer,
            s_5_7,
            s_5_6,
            s_5_8,
            s_5_4,
            s_5_9,
            s_5_5,
        );
        // D s_5_11: write-var gs#914772 <= s_5_10
        fn_state.gs_914772 = s_5_10;
        // N s_5_12: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#914772:bv
        let s_6_0: Bits = fn_state.gs_914772;
        // D s_6_1: cast reint s_6_0 -> u32
        let s_6_1: u32 = (s_6_0.value() as u32);
        // D s_6_2: read-var d:i64
        let s_6_2: i64 = fn_state.d;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: call S_set(s_6_3, s_6_1)
        let s_6_4: () = S_set(state, tracer, s_6_3, s_6_1);
        // N s_6_5: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i64
        let s_7_0: i64 = fn_state.esize;
        // C s_7_1: const #64s : i
        let s_7_1: i128 = 64;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b10 b8
        if s_7_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var m:i64
        let s_8_0: i64 = fn_state.m;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call D_read(s_8_1)
        let s_8_2: u64 = D_read(state, tracer, s_8_1);
        // C s_8_3: const #() : ()
        let s_8_3: () = ();
        // S s_8_4: call FPSCR_read(s_8_3)
        let s_8_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_8_3);
        // C s_8_5: const #32s : i64
        let s_8_5: i64 = 32;
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // D s_8_7: cast zx s_8_2 -> bv
        let s_8_7: Bits = Bits::new(s_8_2 as u128, 64u16);
        // D s_8_8: read-var is_unsigned:u8
        let s_8_8: bool = fn_state.is_unsigned;
        // D s_8_9: read-var rounding:u32
        let s_8_9: u32 = fn_state.rounding;
        // D s_8_10: call FPToFixed(s_8_7, s_8_6, s_8_8, s_8_4, s_8_9, s_8_5)
        let s_8_10: Bits = FPToFixed(
            state,
            tracer,
            s_8_7,
            s_8_6,
            s_8_8,
            s_8_4,
            s_8_9,
            s_8_5,
        );
        // D s_8_11: write-var gs#914778 <= s_8_10
        fn_state.gs_914778 = s_8_10;
        // N s_8_12: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#914778:bv
        let s_9_0: Bits = fn_state.gs_914778;
        // D s_9_1: cast reint s_9_0 -> u32
        let s_9_1: u32 = (s_9_0.value() as u32);
        // D s_9_2: read-var d:i64
        let s_9_2: i64 = fn_state.d;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: call S_set(s_9_3, s_9_1)
        let s_9_4: () = S_set(state, tracer, s_9_3, s_9_1);
        // N s_9_5: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
