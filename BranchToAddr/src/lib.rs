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
use ZeroExtend1::*;
use UsingAArch32::*;
use Hint_Branch::*;
use common::*;
pub fn BranchToAddr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target: Bits,
    branch_type: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_12321: bool,
        target: Bits,
        branch_type: u32,
    }
    let fn_state = FunctionState {
        target,
        branch_type,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var branch_type:u32
        let s_0_0: u32 = fn_state.branch_type;
        // D s_0_1: call Hint_Branch(s_0_0)
        let s_0_1: () = Hint_Branch(state, tracer, s_0_0);
        // D s_0_2: read-var target:bv
        let s_0_2: Bits = fn_state.target;
        // D s_0_3: size-of s_0_2
        let s_0_3: u16 = s_0_2.length();
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #32s : i
        let s_0_5: i128 = 32;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // N s_0_7: branch s_0_6 b6 b1
        if s_0_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var target:bv
        let s_1_0: Bits = fn_state.target;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // C s_1_3: const #64s : i
        let s_1_3: i128 = 64;
        // D s_1_4: cmp-eq s_1_2 s_1_3
        let s_1_4: bool = ((s_1_2) == (s_1_3));
        // N s_1_5: branch s_1_4 b5 b2
        if s_1_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#12321 <= s_2_0
        fn_state.gs_12321 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#12321:u8
        let s_3_0: bool = fn_state.gs_12321;
        // N s_3_1: assert s_3_0
        let s_3_1: () = assert!(s_3_0);
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: read-var target:bv
        let s_3_3: Bits = fn_state.target;
        // C s_3_4: const #1s : i64
        let s_3_4: i64 = 1;
        // C s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // C s_3_6: const #63s : i
        let s_3_6: i128 = 63;
        // C s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: bit-extract s_3_3 s_3_2 s_3_7
        let s_3_8: Bits = (Bits::new(
            ((s_3_3) >> (s_3_2)).value(),
            u16::try_from(s_3_7).unwrap(),
        ));
        // D s_3_9: cast reint s_3_8 -> u64
        let s_3_9: u64 = (s_3_8.value() as u64);
        // C s_3_10: const #12744u : u32
        let s_3_10: u32 = 12744;
        // N s_3_11: write-reg s_3_10 <= s_3_9
        let s_3_11: () = {
            state.write_register::<u64>(s_3_10 as isize, s_3_9);
            tracer.write_register(s_3_10 as isize, s_3_9);
        };
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // C s_4_1: const #14944u : u32
        let s_4_1: u32 = 14944;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call UsingAArch32(s_5_0)
        let s_5_1: bool = UsingAArch32(state, tracer, s_5_0);
        // S s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // D s_5_3: write-var gs#12321 <= s_5_2
        fn_state.gs_12321 = s_5_2;
        // N s_5_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call UsingAArch32(s_6_0)
        let s_6_1: bool = UsingAArch32(state, tracer, s_6_0);
        // N s_6_2: assert s_6_1
        let s_6_2: () = assert!(s_6_1);
        // C s_6_3: const #64s : i
        let s_6_3: i128 = 64;
        // D s_6_4: read-var target:bv
        let s_6_4: Bits = fn_state.target;
        // D s_6_5: call ZeroExtend1(s_6_3, s_6_4)
        let s_6_5: Bits = ZeroExtend1(state, tracer, s_6_3, s_6_4);
        // C s_6_6: const #64s : i
        let s_6_6: i128 = 64;
        // D s_6_7: read-var target:bv
        let s_6_7: Bits = fn_state.target;
        // D s_6_8: bits-cast zx s_6_7 -> bv length s_6_6
        let s_6_8: Bits = s_6_7.zero_extend(s_6_6);
        // D s_6_9: cast reint s_6_8 -> u64
        let s_6_9: u64 = (s_6_8.value() as u64);
        // C s_6_10: const #12744u : u32
        let s_6_10: u32 = 12744;
        // N s_6_11: write-reg s_6_10 <= s_6_9
        let s_6_11: () = {
            state.write_register::<u64>(s_6_10 as isize, s_6_9);
            tracer.write_register(s_6_10 as isize, s_6_9);
        };
        // N s_6_12: jump b4
        return block_4(state, tracer, fn_state);
    }
}
