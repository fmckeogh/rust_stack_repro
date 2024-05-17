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
use D_set::*;
use FPSCR_read::*;
use CheckVFPEnabled::*;
use S_set::*;
use FPConvert__1::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VCVT_ds_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    double_to_single: bool,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_893194: Bits,
        gs_893189: Bits,
        d: i64,
        double_to_single: bool,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        double_to_single,
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
        // D s_1_0: read-var double_to_single:u8
        let s_1_0: bool = fn_state.double_to_single;
        // N s_1_1: branch s_1_0 b4 b2
        if s_1_0 {
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
        // C s_2_3: const #() : ()
        let s_2_3: () = ();
        // S s_2_4: call FPSCR_read(s_2_3)
        let s_2_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_2_3);
        // C s_2_5: const #64s : i64
        let s_2_5: i64 = 64;
        // D s_2_6: cast zx s_2_2 -> bv
        let s_2_6: Bits = Bits::new(s_2_2 as u128, 32u16);
        // D s_2_7: call FPConvert__1(s_2_6, s_2_4, s_2_5)
        let s_2_7: Bits = FPConvert__1(state, tracer, s_2_6, s_2_4, s_2_5);
        // D s_2_8: write-var gs#893189 <= s_2_7
        fn_state.gs_893189 = s_2_7;
        // N s_2_9: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#893189:bv
        let s_3_0: Bits = fn_state.gs_893189;
        // D s_3_1: cast reint s_3_0 -> u64
        let s_3_1: u64 = (s_3_0.value() as u64);
        // D s_3_2: read-var d:i64
        let s_3_2: i64 = fn_state.d;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: call D_set(s_3_3, s_3_1)
        let s_3_4: () = D_set(state, tracer, s_3_3, s_3_1);
        // N s_3_5: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var m:i64
        let s_4_0: i64 = fn_state.m;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call D_read(s_4_1)
        let s_4_2: u64 = D_read(state, tracer, s_4_1);
        // C s_4_3: const #() : ()
        let s_4_3: () = ();
        // S s_4_4: call FPSCR_read(s_4_3)
        let s_4_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_4_3);
        // C s_4_5: const #32s : i64
        let s_4_5: i64 = 32;
        // D s_4_6: cast zx s_4_2 -> bv
        let s_4_6: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_7: call FPConvert__1(s_4_6, s_4_4, s_4_5)
        let s_4_7: Bits = FPConvert__1(state, tracer, s_4_6, s_4_4, s_4_5);
        // D s_4_8: write-var gs#893194 <= s_4_7
        fn_state.gs_893194 = s_4_7;
        // N s_4_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#893194:bv
        let s_5_0: Bits = fn_state.gs_893194;
        // D s_5_1: cast reint s_5_0 -> u32
        let s_5_1: u32 = (s_5_0.value() as u32);
        // D s_5_2: read-var d:i64
        let s_5_2: i64 = fn_state.d;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: call S_set(s_5_3, s_5_1)
        let s_5_4: () = S_set(state, tracer, s_5_3, s_5_1);
        // N s_5_5: return
        return;
    }
}
