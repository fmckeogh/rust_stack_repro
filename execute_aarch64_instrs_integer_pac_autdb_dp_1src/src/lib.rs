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
use HavePACExt::*;
use X_set::*;
use X_read::*;
use SP_read::*;
use AuthDB::*;
use common::*;
pub fn execute_aarch64_instrs_integer_pac_autdb_dp_1src<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
    source_is_sp: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_249637: u64,
        ga_249643: u64,
        d: i64,
        n: i64,
        source_is_sp: bool,
    }
    let fn_state = FunctionState {
        d,
        n,
        source_is_sp,
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
        // S s_0_1: call HavePACExt(s_0_0)
        let s_0_1: bool = HavePACExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var source_is_sp:u8
        let s_2_0: bool = fn_state.source_is_sp;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: read-var d:i64
        let s_3_1: i64 = fn_state.d;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: call X_read(s_3_2, s_3_0)
        let s_3_3: Bits = X_read(state, tracer, s_3_2, s_3_0);
        // D s_3_4: cast reint s_3_3 -> u64
        let s_3_4: u64 = (s_3_3.value() as u64);
        // C s_3_5: const #64s : i64
        let s_3_5: i64 = 64;
        // D s_3_6: read-var n:i64
        let s_3_6: i64 = fn_state.n;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: call X_read(s_3_7, s_3_5)
        let s_3_8: Bits = X_read(state, tracer, s_3_7, s_3_5);
        // D s_3_9: cast reint s_3_8 -> u64
        let s_3_9: u64 = (s_3_8.value() as u64);
        // C s_3_10: const #0u : u8
        let s_3_10: bool = false;
        // D s_3_11: call AuthDB(s_3_4, s_3_9, s_3_10)
        let s_3_11: u64 = AuthDB(state, tracer, s_3_4, s_3_9, s_3_10);
        // D s_3_12: write-var ga#249643 <= s_3_11
        fn_state.ga_249643 = s_3_11;
        // N s_3_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var d:i64
        let s_4_0: i64 = fn_state.d;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var ga#249643:u64
        let s_4_2: u64 = fn_state.ga_249643;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 64u16);
        // C s_4_4: const #64s : i64
        let s_4_4: i64 = 64;
        // D s_4_5: call X_set(s_4_1, s_4_4, s_4_3)
        let s_4_5: () = X_set(state, tracer, s_4_1, s_4_4, s_4_3);
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var d:i64
        let s_5_1: i64 = fn_state.d;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call X_read(s_5_2, s_5_0)
        let s_5_3: Bits = X_read(state, tracer, s_5_2, s_5_0);
        // D s_5_4: cast reint s_5_3 -> u64
        let s_5_4: u64 = (s_5_3.value() as u64);
        // C s_5_5: const #() : ()
        let s_5_5: () = ();
        // S s_5_6: call SP_read(s_5_5)
        let s_5_6: u64 = SP_read(state, tracer, s_5_5);
        // C s_5_7: const #0u : u8
        let s_5_7: bool = false;
        // D s_5_8: call AuthDB(s_5_4, s_5_6, s_5_7)
        let s_5_8: u64 = AuthDB(state, tracer, s_5_4, s_5_6, s_5_7);
        // D s_5_9: write-var ga#249637 <= s_5_8
        fn_state.ga_249637 = s_5_8;
        // N s_5_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var ga#249637:u64
        let s_6_2: u64 = fn_state.ga_249637;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 64u16);
        // C s_6_4: const #64s : i64
        let s_6_4: i64 = 64;
        // D s_6_5: call X_set(s_6_1, s_6_4, s_6_3)
        let s_6_5: () = X_set(state, tracer, s_6_1, s_6_4, s_6_3);
        // N s_6_6: return
        return;
    }
}
