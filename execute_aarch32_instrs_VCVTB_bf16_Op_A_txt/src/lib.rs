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
use S_set::*;
use S_read::*;
use FPConvertBF__1::*;
use common::*;
pub fn execute_aarch32_instrs_VCVTB_bf16_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_367200: u32,
        gs_917427: Bits,
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
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
        // D s_1_0: read-var d:i64
        let s_1_0: i64 = fn_state.d;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call S_read(s_1_1)
        let s_1_2: u32 = S_read(state, tracer, s_1_1);
        // D s_1_3: write-var ga#367200 <= s_1_2
        fn_state.ga_367200 = s_1_2;
        // D s_1_4: read-var m:i64
        let s_1_4: i64 = fn_state.m;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: call S_read(s_1_5)
        let s_1_6: u32 = S_read(state, tracer, s_1_5);
        // C s_1_7: const #() : ()
        let s_1_7: () = ();
        // S s_1_8: call FPSCR_read(s_1_7)
        let s_1_8: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_1_7);
        // D s_1_9: cast zx s_1_6 -> bv
        let s_1_9: Bits = Bits::new(s_1_6 as u128, 32u16);
        // D s_1_10: call FPConvertBF__1(s_1_9, s_1_8)
        let s_1_10: Bits = FPConvertBF__1(state, tracer, s_1_9, s_1_8);
        // D s_1_11: write-var gs#917427 <= s_1_10
        fn_state.gs_917427 = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#917427:bv
        let s_2_0: Bits = fn_state.gs_917427;
        // D s_2_1: cast reint s_2_0 -> u16
        let s_2_1: u16 = (s_2_0.value() as u16);
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: read-var ga#367200:u32
        let s_2_3: u32 = fn_state.ga_367200;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 32u16);
        // D s_2_5: cast zx s_2_1 -> bv
        let s_2_5: Bits = Bits::new(s_2_1 as u128, 16u16);
        // C s_2_6: const #15s : i
        let s_2_6: i128 = 15;
        // C s_2_7: const #1u : u64
        let s_2_7: u64 = 1;
        // C s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 64u16);
        // C s_2_9: lsl s_2_8 s_2_6
        let s_2_9: Bits = s_2_8 << s_2_6;
        // C s_2_10: sub s_2_9 s_2_8
        let s_2_10: Bits = ((s_2_9) - (s_2_8));
        // D s_2_11: and s_2_5 s_2_10
        let s_2_11: Bits = ((s_2_5) & (s_2_10));
        // D s_2_12: lsl s_2_11 s_2_2
        let s_2_12: Bits = s_2_11 << s_2_2;
        // C s_2_13: lsl s_2_10 s_2_2
        let s_2_13: Bits = s_2_10 << s_2_2;
        // C s_2_14: cmpl s_2_13
        let s_2_14: Bits = !s_2_13;
        // D s_2_15: and s_2_4 s_2_14
        let s_2_15: Bits = ((s_2_4) & (s_2_14));
        // D s_2_16: or s_2_15 s_2_12
        let s_2_16: Bits = ((s_2_15) | (s_2_12));
        // D s_2_17: cast reint s_2_16 -> u32
        let s_2_17: u32 = (s_2_16.value() as u32);
        // D s_2_18: read-var d:i64
        let s_2_18: i64 = fn_state.d;
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: call S_set(s_2_19, s_2_17)
        let s_2_20: () = S_set(state, tracer, s_2_19, s_2_17);
        // N s_2_21: return
        return;
    }
}
