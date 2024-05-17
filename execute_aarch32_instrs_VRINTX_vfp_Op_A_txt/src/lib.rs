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
use CheckVFPEnabled::*;
use FPRoundingMode::*;
use FPSCR_read::*;
use D_set::*;
use Zeros::*;
use FPRoundInt::*;
use S_read::*;
use S_set::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VRINTX_vfp_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    exact: bool,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_915414: Bits,
        gs_915407: Bits,
        rounding: u32,
        ga_366002: u16,
        gs_915419: Bits,
        d: i64,
        esize: i64,
        exact: bool,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        esize,
        exact,
        m,
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call FPSCR_read(s_1_0)
        let s_1_1: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_1_0);
        // S s_1_2: call FPRoundingMode(s_1_1)
        let s_1_2: u32 = FPRoundingMode(state, tracer, s_1_1);
        // D s_1_3: write-var rounding <= s_1_2
        fn_state.rounding = s_1_2;
        // D s_1_4: read-var esize:i64
        let s_1_4: i64 = fn_state.esize;
        // C s_1_5: const #16s : i
        let s_1_5: i128 = 16;
        // D s_1_6: cast zx s_1_4 -> i
        let s_1_6: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_7: cmp-eq s_1_6 s_1_5
        let s_1_7: bool = ((s_1_6) == (s_1_5));
        // D s_1_8: not s_1_7
        let s_1_8: bool = !s_1_7;
        // N s_1_9: branch s_1_8 b4 b2
        if s_1_8 {
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
        // C s_2_0: const #16s : i
        let s_2_0: i128 = 16;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u16
        let s_2_2: u16 = (s_2_1.value() as u16);
        // D s_2_3: write-var ga#366002 <= s_2_2
        fn_state.ga_366002 = s_2_2;
        // D s_2_4: read-var m:i64
        let s_2_4: i64 = fn_state.m;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: call S_read(s_2_5)
        let s_2_6: u32 = S_read(state, tracer, s_2_5);
        // C s_2_7: const #0s : i
        let s_2_7: i128 = 0;
        // D s_2_8: cast zx s_2_6 -> bv
        let s_2_8: Bits = Bits::new(s_2_6 as u128, 32u16);
        // C s_2_9: const #1s : i64
        let s_2_9: i64 = 1;
        // C s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // C s_2_11: const #15s : i
        let s_2_11: i128 = 15;
        // C s_2_12: add s_2_11 s_2_10
        let s_2_12: i128 = (s_2_11 + s_2_10);
        // D s_2_13: bit-extract s_2_8 s_2_7 s_2_12
        let s_2_13: Bits = (Bits::new(
            ((s_2_8) >> (s_2_7)).value(),
            u16::try_from(s_2_12).unwrap(),
        ));
        // D s_2_14: cast reint s_2_13 -> u16
        let s_2_14: u16 = (s_2_13.value() as u16);
        // C s_2_15: const #() : ()
        let s_2_15: () = ();
        // S s_2_16: call FPSCR_read(s_2_15)
        let s_2_16: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_2_15);
        // D s_2_17: cast zx s_2_14 -> bv
        let s_2_17: Bits = Bits::new(s_2_14 as u128, 16u16);
        // D s_2_18: read-var rounding:u32
        let s_2_18: u32 = fn_state.rounding;
        // D s_2_19: read-var exact:u8
        let s_2_19: bool = fn_state.exact;
        // D s_2_20: call FPRoundInt(s_2_17, s_2_16, s_2_18, s_2_19)
        let s_2_20: Bits = FPRoundInt(state, tracer, s_2_17, s_2_16, s_2_18, s_2_19);
        // D s_2_21: write-var gs#915407 <= s_2_20
        fn_state.gs_915407 = s_2_20;
        // N s_2_22: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#915407:bv
        let s_3_0: Bits = fn_state.gs_915407;
        // D s_3_1: cast reint s_3_0 -> u16
        let s_3_1: u16 = (s_3_0.value() as u16);
        // D s_3_2: read-var ga#366002:u16
        let s_3_2: u16 = fn_state.ga_366002;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 16u16);
        // D s_3_4: cast zx s_3_1 -> bv
        let s_3_4: Bits = Bits::new(s_3_1 as u128, 16u16);
        // D s_3_5: cast reint s_3_3 -> u128
        let s_3_5: u128 = (s_3_3.value() as u128);
        // D s_3_6: size-of s_3_3
        let s_3_6: u16 = s_3_3.length();
        // D s_3_7: cast reint s_3_4 -> u128
        let s_3_7: u128 = (s_3_4.value() as u128);
        // D s_3_8: size-of s_3_4
        let s_3_8: u16 = s_3_4.length();
        // D s_3_9: lsl s_3_5 s_3_8
        let s_3_9: u128 = s_3_5 << s_3_8;
        // D s_3_10: or s_3_9 s_3_7
        let s_3_10: u128 = ((s_3_9) | (s_3_7));
        // D s_3_11: add s_3_6 s_3_8
        let s_3_11: u16 = (s_3_6 + s_3_8);
        // D s_3_12: create-bits s_3_10 s_3_11
        let s_3_12: Bits = Bits::new(s_3_10, s_3_11);
        // D s_3_13: cast reint s_3_12 -> u32
        let s_3_13: u32 = (s_3_12.value() as u32);
        // D s_3_14: read-var d:i64
        let s_3_14: i64 = fn_state.d;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: call S_set(s_3_15, s_3_13)
        let s_3_16: () = S_set(state, tracer, s_3_15, s_3_13);
        // N s_3_17: return
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
        // D s_5_5: cast zx s_5_2 -> bv
        let s_5_5: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_6: read-var rounding:u32
        let s_5_6: u32 = fn_state.rounding;
        // D s_5_7: read-var exact:u8
        let s_5_7: bool = fn_state.exact;
        // D s_5_8: call FPRoundInt(s_5_5, s_5_4, s_5_6, s_5_7)
        let s_5_8: Bits = FPRoundInt(state, tracer, s_5_5, s_5_4, s_5_6, s_5_7);
        // D s_5_9: write-var gs#915414 <= s_5_8
        fn_state.gs_915414 = s_5_8;
        // N s_5_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#915414:bv
        let s_6_0: Bits = fn_state.gs_915414;
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
        // D s_8_5: cast zx s_8_2 -> bv
        let s_8_5: Bits = Bits::new(s_8_2 as u128, 64u16);
        // D s_8_6: read-var rounding:u32
        let s_8_6: u32 = fn_state.rounding;
        // D s_8_7: read-var exact:u8
        let s_8_7: bool = fn_state.exact;
        // D s_8_8: call FPRoundInt(s_8_5, s_8_4, s_8_6, s_8_7)
        let s_8_8: Bits = FPRoundInt(state, tracer, s_8_5, s_8_4, s_8_6, s_8_7);
        // D s_8_9: write-var gs#915419 <= s_8_8
        fn_state.gs_915419 = s_8_8;
        // N s_8_10: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#915419:bv
        let s_9_0: Bits = fn_state.gs_915419;
        // D s_9_1: cast reint s_9_0 -> u64
        let s_9_1: u64 = (s_9_0.value() as u64);
        // D s_9_2: read-var d:i64
        let s_9_2: i64 = fn_state.d;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: call D_set(s_9_3, s_9_1)
        let s_9_4: () = D_set(state, tracer, s_9_3, s_9_1);
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
