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
use execute_aarch64_instrs_integer_arithmetic_rev::*;
use Unreachable::*;
use common::*;
pub fn decode_rev32_int_aarch64_instrs_integer_arithmetic_rev<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    opc: u8,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        container_size: i64,
        datasize: i64,
        n: i64,
        d: i64,
        ga_264495: i64,
        Rd: u8,
        Rn: u8,
        opc: u8,
        sf: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        opc,
        sf,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // D s_0_10: read-var sf:u8
        let s_0_10: bool = fn_state.sf;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b12 b1
        if s_0_14 {
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
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#264495 <= s_1_0
        fn_state.ga_264495 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#264495:i64
        let s_2_0: i64 = fn_state.ga_264495;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // C s_2_2: const #16s : i64
        let s_2_2: i64 = 16;
        // D s_2_3: write-var container_size <= s_2_2
        fn_state.container_size = s_2_2;
        // D s_2_4: read-var opc:u8
        let s_2_4: u8 = fn_state.opc;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // C s_2_6: const #0u : u8
        let s_2_6: u8 = 0;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // D s_2_8: cmp-eq s_2_5 s_2_7
        let s_2_8: bool = ((s_2_5) == (s_2_7));
        // D s_2_9: not s_2_8
        let s_2_9: bool = !s_2_8;
        // N s_2_10: branch s_2_9 b5 b3
        if s_2_9 {
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
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call Unreachable(s_3_0)
        let s_3_1: () = Unreachable(state, tracer, s_3_0);
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var container_size:i64
        let s_4_0: i64 = fn_state.container_size;
        // D s_4_1: read-var datasize:i64
        let s_4_1: i64 = fn_state.datasize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var d:i64
        let s_4_4: i64 = fn_state.d;
        // D s_4_5: read-var n:i64
        let s_4_5: i64 = fn_state.n;
        // D s_4_6: call execute_aarch64_instrs_integer_arithmetic_rev(s_4_0, s_4_4, s_4_3, s_4_5)
        let s_4_6: () = execute_aarch64_instrs_integer_arithmetic_rev(
            state,
            tracer,
            s_4_0,
            s_4_4,
            s_4_3,
            s_4_5,
        );
        // N s_4_7: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var opc:u8
        let s_5_0: u8 = fn_state.opc;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
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
        // C s_6_0: const #16s : i64
        let s_6_0: i64 = 16;
        // D s_6_1: write-var container_size <= s_6_0
        fn_state.container_size = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var opc:u8
        let s_7_0: u8 = fn_state.opc;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
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
        // C s_8_0: const #32s : i64
        let s_8_0: i64 = 32;
        // D s_8_1: write-var container_size <= s_8_0
        fn_state.container_size = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var sf:u8
        let s_9_0: bool = fn_state.sf;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #0u : u8
        let s_9_2: bool = false;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
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
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // D s_10_1: write-var container_size <= s_10_0
        fn_state.container_size = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // D s_12_1: write-var ga#264495 <= s_12_0
        fn_state.ga_264495 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
