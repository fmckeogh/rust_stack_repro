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
use execute_aarch64_instrs_branch_unconditional_eret::*;
use common::*;
pub fn decode_ereta_aarch64_instrs_branch_unconditional_eret<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op4: u8,
    Rn: u8,
    M: bool,
    A: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_148101: bool,
        gs_148104: bool,
        gs_148105: bool,
        use_key_a: bool,
        pac: bool,
        op4: u8,
        Rn: u8,
        M: bool,
        A: bool,
    }
    let fn_state = FunctionState {
        op4,
        Rn,
        M,
        A,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
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
        // N s_0_7: branch s_0_6 b17 b1
        if s_0_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var A:u8
        let s_1_0: bool = fn_state.A;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // D s_1_5: write-var pac <= s_1_4
        fn_state.pac = s_1_4;
        // D s_1_6: read-var M:u8
        let s_1_6: bool = fn_state.M;
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 1u16);
        // C s_1_8: const #0u : u8
        let s_1_8: bool = false;
        // C s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 1u16);
        // D s_1_10: cmp-eq s_1_7 s_1_9
        let s_1_10: bool = ((s_1_7) == (s_1_9));
        // D s_1_11: write-var use_key_a <= s_1_10
        fn_state.use_key_a = s_1_10;
        // D s_1_12: read-var pac:u8
        let s_1_12: bool = fn_state.pac;
        // D s_1_13: not s_1_12
        let s_1_13: bool = !s_1_12;
        // N s_1_14: branch s_1_13 b16 b2
        if s_1_13 {
            return block_16(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#148101 <= s_2_0
        fn_state.gs_148101 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#148101:u8
        let s_3_0: bool = fn_state.gs_148101;
        // N s_3_1: branch s_3_0 b15 b4
        if s_3_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var pac:u8
        let s_4_0: bool = fn_state.pac;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#148105 <= s_5_0
        fn_state.gs_148105 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#148105:u8
        let s_6_0: bool = fn_state.gs_148105;
        // N s_6_1: branch s_6_0 b10 b7
        if s_6_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var Rn:u8
        let s_7_0: u8 = fn_state.Rn;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 5u16);
        // C s_7_2: const #31u : u8
        let s_7_2: u8 = 31;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 5u16);
        // D s_7_4: cmp-ne s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) != (s_7_3));
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
        // D s_8_0: read-var pac:u8
        let s_8_0: bool = fn_state.pac;
        // D s_8_1: read-var use_key_a:u8
        let s_8_1: bool = fn_state.use_key_a;
        // D s_8_2: call execute_aarch64_instrs_branch_unconditional_eret(s_8_0, s_8_1)
        let s_8_2: () = execute_aarch64_instrs_branch_unconditional_eret(
            state,
            tracer,
            s_8_0,
            s_8_1,
        );
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HavePACExt(s_11_0)
        let s_11_1: bool = HavePACExt(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // N s_11_3: branch s_11_2 b14 b12
        if s_11_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var op4:u8
        let s_12_0: u8 = fn_state.op4;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 5u16);
        // C s_12_2: const #31u : u8
        let s_12_2: u8 = 31;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 5u16);
        // D s_12_4: cmp-ne s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) != (s_12_3));
        // D s_12_5: write-var gs#148104 <= s_12_4
        fn_state.gs_148104 = s_12_4;
        // N s_12_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#148104:u8
        let s_13_0: bool = fn_state.gs_148104;
        // D s_13_1: write-var gs#148105 <= s_13_0
        fn_state.gs_148105 = s_13_0;
        // N s_13_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#148104 <= s_14_0
        fn_state.gs_148104 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var op4:u8
        let s_16_0: u8 = fn_state.op4;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 5u16);
        // C s_16_2: const #0u : u8
        let s_16_2: u8 = 0;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 5u16);
        // D s_16_4: cmp-ne s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) != (s_16_3));
        // D s_16_5: write-var gs#148101 <= s_16_4
        fn_state.gs_148101 = s_16_4;
        // N s_16_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
}
