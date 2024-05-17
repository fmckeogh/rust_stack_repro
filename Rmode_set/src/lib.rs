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
use BadMode::*;
use LookUpRIndex::*;
use ConstrainUnpredictableBool::*;
use HaveAArch64::*;
use CurrentSecurityState::*;
use set_R_bits::*;
use set_R::*;
use common::*;
pub fn Rmode_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    mode: u8,
    value_name: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_8992: bool,
        gs_8994: bool,
        n: i128,
        mode: u8,
        value_name: u32,
    }
    let fn_state = FunctionState {
        n,
        mode,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var n:i
        let s_0_1: i128 = fn_state.n;
        // D s_0_2: cmp-ge s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) >= (s_0_0));
        // N s_0_3: branch s_0_2 b17 b1
        if s_0_2 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#8992 <= s_1_0
        fn_state.gs_8992 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#8992:u8
        let s_2_0: bool = fn_state.gs_8992;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call CurrentSecurityState(s_2_2)
        let s_2_3: u32 = CurrentSecurityState(state, tracer, s_2_2);
        // C s_2_4: const #3u : u32
        let s_2_4: u32 = 3;
        // S s_2_5: cmp-eq s_2_3 s_2_4
        let s_2_5: bool = ((s_2_3) == (s_2_4));
        // N s_2_6: branch s_2_5 b16 b3
        if s_2_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var mode:u8
        let s_4_0: u8 = fn_state.mode;
        // D s_4_1: call BadMode(s_4_0)
        let s_4_1: bool = BadMode(state, tracer, s_4_0);
        // D s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // N s_4_3: assert s_4_2
        let s_4_3: () = assert!(s_4_2);
        // D s_4_4: read-var mode:u8
        let s_4_4: u8 = fn_state.mode;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 5u16);
        // C s_4_6: const #384u : u32
        let s_4_6: u32 = 384;
        // D s_4_7: read-reg s_4_6:u8
        let s_4_7: u8 = {
            let value = state.read_register::<u8>(s_4_6 as isize);
            tracer.read_register(s_4_6 as isize, value);
            value
        };
        // D s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 5u16);
        // D s_4_9: cmp-eq s_4_5 s_4_8
        let s_4_9: bool = ((s_4_5) == (s_4_8));
        // N s_4_10: branch s_4_9 b11 b5
        if s_4_9 {
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveAArch64(s_5_0)
        let s_5_1: bool = HaveAArch64(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b10 b6
        if s_5_1 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#8994 <= s_6_0
        fn_state.gs_8994 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#8994:u8
        let s_7_0: bool = fn_state.gs_8994;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
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
        // D s_8_0: read-var n:i
        let s_8_0: i128 = fn_state.n;
        // D s_8_1: cast reint s_8_0 -> i64
        let s_8_1: i64 = (s_8_0 as i64);
        // D s_8_2: read-var mode:u8
        let s_8_2: u8 = fn_state.mode;
        // D s_8_3: call LookUpRIndex(s_8_1, s_8_2)
        let s_8_3: i64 = LookUpRIndex(state, tracer, s_8_1, s_8_2);
        // C s_8_4: const #31s : i64
        let s_8_4: i64 = 31;
        // C s_8_5: const #0s : i64
        let s_8_5: i64 = 0;
        // D s_8_6: read-var value_name:u32
        let s_8_6: u32 = fn_state.value_name;
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 32u16);
        // D s_8_8: call set_R_bits(s_8_3, s_8_4, s_8_5, s_8_7)
        let s_8_8: () = set_R_bits(state, tracer, s_8_3, s_8_4, s_8_5, s_8_7);
        // N s_8_9: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var n:i
        let s_9_0: i128 = fn_state.n;
        // D s_9_1: cast reint s_9_0 -> i64
        let s_9_1: i64 = (s_9_0 as i64);
        // D s_9_2: read-var mode:u8
        let s_9_2: u8 = fn_state.mode;
        // D s_9_3: call LookUpRIndex(s_9_1, s_9_2)
        let s_9_3: i64 = LookUpRIndex(state, tracer, s_9_1, s_9_2);
        // C s_9_4: const #64s : i
        let s_9_4: i128 = 64;
        // D s_9_5: read-var value_name:u32
        let s_9_5: u32 = fn_state.value_name;
        // D s_9_6: cast zx s_9_5 -> bv
        let s_9_6: Bits = Bits::new(s_9_5 as u128, 32u16);
        // D s_9_7: bits-cast zx s_9_6 -> bv length s_9_4
        let s_9_7: Bits = s_9_6.zero_extend(s_9_4);
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // D s_9_9: call set_R(s_9_3, s_9_8)
        let s_9_9: () = set_R(state, tracer, s_9_3, s_9_8);
        // N s_9_10: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #45u : u32
        let s_10_0: u32 = 45;
        // S s_10_1: call ConstrainUnpredictableBool(s_10_0)
        let s_10_1: bool = ConstrainUnpredictableBool(state, tracer, s_10_0);
        // D s_10_2: write-var gs#8994 <= s_10_1
        fn_state.gs_8994 = s_10_1;
        // N s_10_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #13s : i
        let s_11_0: i128 = 13;
        // D s_11_1: read-var n:i
        let s_11_1: i128 = fn_state.n;
        // D s_11_2: cmp-eq s_11_1 s_11_0
        let s_11_2: bool = ((s_11_1) == (s_11_0));
        // N s_11_3: branch s_11_2 b15 b12
        if s_11_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #14s : i
        let s_12_0: i128 = 14;
        // D s_12_1: read-var n:i
        let s_12_1: i128 = fn_state.n;
        // D s_12_2: cmp-eq s_12_1 s_12_0
        let s_12_2: bool = ((s_12_1) == (s_12_0));
        // N s_12_3: branch s_12_2 b14 b13
        if s_12_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i
        let s_13_0: i128 = fn_state.n;
        // D s_13_1: cast reint s_13_0 -> i64
        let s_13_1: i64 = (s_13_0 as i64);
        // C s_13_2: const #31s : i64
        let s_13_2: i64 = 31;
        // C s_13_3: const #0s : i64
        let s_13_3: i64 = 0;
        // D s_13_4: read-var value_name:u32
        let s_13_4: u32 = fn_state.value_name;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 32u16);
        // D s_13_6: call set_R_bits(s_13_1, s_13_2, s_13_3, s_13_5)
        let s_13_6: () = set_R_bits(state, tracer, s_13_1, s_13_2, s_13_3, s_13_5);
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var value_name:u32
        let s_14_0: u32 = fn_state.value_name;
        // C s_14_1: const #11696u : u32
        let s_14_1: u32 = 11696;
        // N s_14_2: write-reg s_14_1 <= s_14_0
        let s_14_2: () = {
            state.write_register::<u32>(s_14_1 as isize, s_14_0);
            tracer.write_register(s_14_1 as isize, s_14_0);
        };
        // N s_14_3: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var value_name:u32
        let s_15_0: u32 = fn_state.value_name;
        // C s_15_1: const #15360u : u32
        let s_15_1: u32 = 15360;
        // N s_15_2: write-reg s_15_1 <= s_15_0
        let s_15_2: () = {
            state.write_register::<u32>(s_15_1 as isize, s_15_0);
            tracer.write_register(s_15_1 as isize, s_15_0);
        };
        // N s_15_3: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var mode:u8
        let s_16_0: u8 = fn_state.mode;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 5u16);
        // C s_16_2: const #384u : u32
        let s_16_2: u32 = 384;
        // D s_16_3: read-reg s_16_2:u8
        let s_16_3: u8 = {
            let value = state.read_register::<u8>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 5u16);
        // D s_16_5: cmp-ne s_16_1 s_16_4
        let s_16_5: bool = ((s_16_1) != (s_16_4));
        // N s_16_6: assert s_16_5
        let s_16_6: () = assert!(s_16_5);
        // N s_16_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #14s : i
        let s_17_0: i128 = 14;
        // D s_17_1: read-var n:i
        let s_17_1: i128 = fn_state.n;
        // D s_17_2: cmp-le s_17_1 s_17_0
        let s_17_2: bool = ((s_17_1) <= (s_17_0));
        // D s_17_3: write-var gs#8992 <= s_17_2
        fn_state.gs_8992 = s_17_2;
        // N s_17_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
