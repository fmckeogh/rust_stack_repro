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
use u_get_PMBSR_EL1_Type_S::*;
use SetInterruptRequestLevel::*;
use HaveStatisticalProfiling::*;
use common::*;
pub fn SPECycle<T: Tracer>(state: &mut State, tracer: &T, gs_26245: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_26250: i64,
        i: i64,
        ga_20488: u32,
        gs_26245: (),
    }
    let fn_state = FunctionState {
        gs_26245,
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
        // S s_0_1: call HaveStatisticalProfiling(s_0_0)
        let s_0_1: bool = HaveStatisticalProfiling(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b14 b1
        if s_0_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #22416u : u32
        let s_1_0: u32 = 22416;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: bool = {
            let value = state.read_register::<bool>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // N s_1_2: branch s_1_1 b7 b2
        if s_1_1 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #13704u : u32
        let s_3_0: u32 = 13704;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_PMBSR_EL1_Type_S(s_3_1)
        let s_3_2: bool = u_get_PMBSR_EL1_Type_S(state, tracer, s_3_1);
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // C s_3_4: const #1u : u8
        let s_3_4: bool = true;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // N s_3_7: branch s_3_6 b6 b4
        if s_3_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u32
        let s_4_0: u32 = 0;
        // D s_4_1: write-var ga#20488 <= s_4_0
        fn_state.ga_20488 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #12u : u32
        let s_5_0: u32 = 12;
        // D s_5_1: read-var ga#20488:u32
        let s_5_1: u32 = fn_state.ga_20488;
        // D s_5_2: call SetInterruptRequestLevel(s_5_0, s_5_1)
        let s_5_2: () = SetInterruptRequestLevel(state, tracer, s_5_0, s_5_1);
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: write-var ga#20488 <= s_6_0
        fn_state.ga_20488 = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i64
        let s_7_0: i64 = 0;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // C s_7_2: const #1000u : u32
        let s_7_2: u32 = 1000;
        // D s_7_3: read-reg s_7_2:i64
        let s_7_3: i64 = {
            let value = state.read_register::<i64>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: sub s_7_4 s_7_1
        let s_7_5: i128 = ((s_7_4) - (s_7_1));
        // D s_7_6: cast reint s_7_5 -> i64
        let s_7_6: i64 = (s_7_5 as i64);
        // D s_7_7: write-var gs#26250 <= s_7_6
        fn_state.gs_26250 = s_7_6;
        // D s_7_8: write-var i <= s_7_0
        fn_state.i = s_7_0;
        // N s_7_9: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var i:i64
        let s_8_0: i64 = fn_state.i;
        // D s_8_1: read-var gs#26250:i64
        let s_8_1: i64 = fn_state.gs_26250;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b13 b9
        if s_8_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #102648u : u32
        let s_9_0: u32 = 102648;
        // D s_9_1: read-reg s_9_0:[u8; 32]
        let s_9_1: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: read-var i:i64
        let s_9_2: i64 = fn_state.i;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-element s_9_1[s_9_3]
        let s_9_4: bool = s_9_1[(s_9_3) as usize];
        // N s_9_5: branch s_9_4 b12 b10
        if s_9_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var i:i64
        let s_11_0: i64 = fn_state.i;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var i <= s_11_2
        fn_state.i = s_11_2;
        // N s_11_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #18464u : u32
        let s_12_0: u32 = 18464;
        // D s_12_1: read-reg s_12_0:[i; 32]
        let s_12_1: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: read-var i:i64
        let s_12_2: i64 = fn_state.i;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: read-element s_12_1[s_12_3]
        let s_12_4: i128 = s_12_1[(s_12_3) as usize];
        // C s_12_5: const #1s : i
        let s_12_5: i128 = 1;
        // D s_12_6: add s_12_4 s_12_5
        let s_12_6: i128 = (s_12_4 + s_12_5);
        // C s_12_7: const #18464u : u32
        let s_12_7: u32 = 18464;
        // D s_12_8: read-reg s_12_7:[i; 32]
        let s_12_8: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_12_7 as isize);
            tracer.read_register(s_12_7 as isize, value);
            value
        };
        // D s_12_9: read-var i:i64
        let s_12_9: i64 = fn_state.i;
        // D s_12_10: cast zx s_12_9 -> i
        let s_12_10: i128 = (i128::try_from(s_12_9).unwrap());
        // D s_12_11: mutate-element s_12_8[s_12_10] <= s_12_6
        let s_12_11: [i128; 32usize] = {
            let mut local = s_12_8.clone();
            local[(s_12_10) as usize] = s_12_6;
            local
        };
        // D s_12_12: cast cvt s_12_11 -> [i; 0]
        let s_12_12: alloc::vec::Vec<i128> = alloc::vec::Vec::from(s_12_11);
        // D s_12_13: cast cvt s_12_12 -> [i; 32]
        let s_12_13: [i128; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_12_12);
            buf
        };
        // C s_12_14: const #18464u : u32
        let s_12_14: u32 = 18464;
        // N s_12_15: write-reg s_12_14 <= s_12_13
        let s_12_15: () = {
            state.write_register::<[i128; 32usize]>(s_12_14 as isize, s_12_13);
            tracer.write_register(s_12_14 as isize, s_12_13);
        };
        // N s_12_16: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
}
