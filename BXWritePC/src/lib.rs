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
use AArch32_CurrentCond::*;
use Bit::*;
use BranchTo::*;
use SelectInstrSet::*;
use ConstrainUnpredictableBool::*;
use common::*;
pub fn BXWritePC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address_in: u32,
    branch_type: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31402: bool,
        gs_31408: bool,
        address: u32,
        address_in: u32,
        branch_type: u32,
    }
    let fn_state = FunctionState {
        address_in,
        branch_type,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var address_in:u32
        let s_0_0: u32 = fn_state.address_in;
        // D s_0_1: write-var address <= s_0_0
        fn_state.address = s_0_0;
        // C s_0_2: const #0s : i
        let s_0_2: i128 = 0;
        // D s_0_3: read-var address:u32
        let s_0_3: u32 = fn_state.address;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 32u16);
        // C s_0_5: const #1u : u64
        let s_0_5: u64 = 1;
        // D s_0_6: bit-extract s_0_4 s_0_2 s_0_5
        let s_0_6: Bits = (Bits::new(
            ((s_0_4) >> (s_0_2)).value(),
            u16::try_from(s_0_5).unwrap(),
        ));
        // D s_0_7: cast reint s_0_6 -> u8
        let s_0_7: bool = ((s_0_6.value()) != 0);
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // C s_0_9: const #0u : u64
        let s_0_9: u64 = 0;
        // D s_0_10: cast zx s_0_7 -> u64
        let s_0_10: u64 = (s_0_7 as u64);
        // C s_0_11: const #1u : u64
        let s_0_11: u64 = 1;
        // D s_0_12: and s_0_10 s_0_11
        let s_0_12: u64 = ((s_0_10) & (s_0_11));
        // D s_0_13: cmp-eq s_0_12 s_0_11
        let s_0_13: bool = ((s_0_12) == (s_0_11));
        // D s_0_14: lsl s_0_10 s_0_8
        let s_0_14: u64 = s_0_10 << s_0_8;
        // D s_0_15: or s_0_9 s_0_14
        let s_0_15: u64 = ((s_0_9) | (s_0_14));
        // D s_0_16: cmpl s_0_14
        let s_0_16: u64 = !s_0_14;
        // D s_0_17: and s_0_9 s_0_16
        let s_0_17: u64 = ((s_0_9) & (s_0_16));
        // D s_0_18: select s_0_13 s_0_15 s_0_17
        let s_0_18: u64 = if s_0_13 { s_0_15 } else { s_0_17 };
        // D s_0_19: cast trunc s_0_18 -> u8
        let s_0_19: bool = ((s_0_18) != 0);
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 1u16);
        // C s_0_21: const #1u : u8
        let s_0_21: bool = true;
        // C s_0_22: cast zx s_0_21 -> bv
        let s_0_22: Bits = Bits::new(s_0_21 as u128, 1u16);
        // D s_0_23: cmp-eq s_0_20 s_0_22
        let s_0_23: bool = ((s_0_20) == (s_0_22));
        // N s_0_24: branch s_0_23 b12 b1
        if s_0_23 {
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
        // C s_1_0: const #1u : u32
        let s_1_0: u32 = 1;
        // S s_1_1: call SelectInstrSet(s_1_0)
        let s_1_1: () = SelectInstrSet(state, tracer, s_1_0);
        // C s_1_2: const #1s : i
        let s_1_2: i128 = 1;
        // D s_1_3: read-var address:u32
        let s_1_3: u32 = fn_state.address;
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 32u16);
        // C s_1_5: const #1u : u64
        let s_1_5: u64 = 1;
        // D s_1_6: bit-extract s_1_4 s_1_2 s_1_5
        let s_1_6: Bits = (Bits::new(
            ((s_1_4) >> (s_1_2)).value(),
            u16::try_from(s_1_5).unwrap(),
        ));
        // D s_1_7: cast reint s_1_6 -> u8
        let s_1_7: bool = ((s_1_6.value()) != 0);
        // C s_1_8: const #0s : i
        let s_1_8: i128 = 0;
        // C s_1_9: const #0u : u64
        let s_1_9: u64 = 0;
        // D s_1_10: cast zx s_1_7 -> u64
        let s_1_10: u64 = (s_1_7 as u64);
        // C s_1_11: const #1u : u64
        let s_1_11: u64 = 1;
        // D s_1_12: and s_1_10 s_1_11
        let s_1_12: u64 = ((s_1_10) & (s_1_11));
        // D s_1_13: cmp-eq s_1_12 s_1_11
        let s_1_13: bool = ((s_1_12) == (s_1_11));
        // D s_1_14: lsl s_1_10 s_1_8
        let s_1_14: u64 = s_1_10 << s_1_8;
        // D s_1_15: or s_1_9 s_1_14
        let s_1_15: u64 = ((s_1_9) | (s_1_14));
        // D s_1_16: cmpl s_1_14
        let s_1_16: u64 = !s_1_14;
        // D s_1_17: and s_1_9 s_1_16
        let s_1_17: u64 = ((s_1_9) & (s_1_16));
        // D s_1_18: select s_1_13 s_1_15 s_1_17
        let s_1_18: u64 = if s_1_13 { s_1_15 } else { s_1_17 };
        // D s_1_19: cast trunc s_1_18 -> u8
        let s_1_19: bool = ((s_1_18) != 0);
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // C s_1_21: const #1u : u8
        let s_1_21: bool = true;
        // C s_1_22: cast zx s_1_21 -> bv
        let s_1_22: Bits = Bits::new(s_1_21 as u128, 1u16);
        // D s_1_23: cmp-eq s_1_20 s_1_22
        let s_1_23: bool = ((s_1_20) == (s_1_22));
        // N s_1_24: branch s_1_23 b11 b2
        if s_1_23 {
            return block_11(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#31402 <= s_2_0
        fn_state.gs_31402 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#31402:u8
        let s_3_0: bool = fn_state.gs_31402;
        // N s_3_1: branch s_3_0 b10 b4
        if s_3_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call AArch32_CurrentCond(s_6_0)
        let s_6_1: u8 = AArch32_CurrentCond(state, tracer, s_6_0);
        // C s_6_2: const #1s : i
        let s_6_2: i128 = 1;
        // S s_6_3: cast zx s_6_1 -> bv
        let s_6_3: Bits = Bits::new(s_6_1 as u128, 4u16);
        // C s_6_4: const #1s : i64
        let s_6_4: i64 = 1;
        // C s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // C s_6_6: const #2s : i
        let s_6_6: i128 = 2;
        // C s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: bit-extract s_6_3 s_6_2 s_6_7
        let s_6_8: Bits = (Bits::new(
            ((s_6_3) >> (s_6_2)).value(),
            u16::try_from(s_6_7).unwrap(),
        ));
        // D s_6_9: cast reint s_6_8 -> u8
        let s_6_9: u8 = (s_6_8.value() as u8);
        // D s_6_10: cast zx s_6_9 -> bv
        let s_6_10: Bits = Bits::new(s_6_9 as u128, 3u16);
        // C s_6_11: const #7u : u8
        let s_6_11: u8 = 7;
        // C s_6_12: cast zx s_6_11 -> bv
        let s_6_12: Bits = Bits::new(s_6_11 as u128, 3u16);
        // D s_6_13: cmp-eq s_6_10 s_6_12
        let s_6_13: bool = ((s_6_10) == (s_6_12));
        // D s_6_14: not s_6_13
        let s_6_14: bool = !s_6_13;
        // N s_6_15: branch s_6_14 b9 b7
        if s_6_14 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#31408 <= s_7_0
        fn_state.gs_31408 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#31408:u8
        let s_8_0: bool = fn_state.gs_31408;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // D s_8_2: read-var address:u32
        let s_8_2: u32 = fn_state.address;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 32u16);
        // D s_8_4: read-var branch_type:u32
        let s_8_4: u32 = fn_state.branch_type;
        // D s_8_5: call BranchTo(s_8_3, s_8_4, s_8_1)
        let s_8_5: () = BranchTo(state, tracer, s_8_3, s_8_4, s_8_1);
        // N s_8_6: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#31408 <= s_9_0
        fn_state.gs_31408 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // S s_10_1: call Bit(s_10_0)
        let s_10_1: bool = Bit(state, tracer, s_10_0);
        // C s_10_2: const #1s : i
        let s_10_2: i128 = 1;
        // D s_10_3: read-var address:u32
        let s_10_3: u32 = fn_state.address;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 32u16);
        // C s_10_5: const #1u : u64
        let s_10_5: u64 = 1;
        // D s_10_6: bit-insert s_10_4 s_10_4 s_10_2 s_10_5
        let s_10_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_10_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_10_4.length(),
            );
            (s_10_4 & mask) | (s_10_4 << s_10_2)
        };
        // D s_10_7: cast reint s_10_6 -> u32
        let s_10_7: u32 = (s_10_6.value() as u32);
        // D s_10_8: write-var address <= s_10_7
        fn_state.address = s_10_7;
        // N s_10_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #47u : u32
        let s_11_0: u32 = 47;
        // S s_11_1: call ConstrainUnpredictableBool(s_11_0)
        let s_11_1: bool = ConstrainUnpredictableBool(state, tracer, s_11_0);
        // D s_11_2: write-var gs#31402 <= s_11_1
        fn_state.gs_31402 = s_11_1;
        // N s_11_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2u : u32
        let s_12_0: u32 = 2;
        // S s_12_1: call SelectInstrSet(s_12_0)
        let s_12_1: () = SelectInstrSet(state, tracer, s_12_0);
        // C s_12_2: const #0u : u8
        let s_12_2: bool = false;
        // S s_12_3: call Bit(s_12_2)
        let s_12_3: bool = Bit(state, tracer, s_12_2);
        // C s_12_4: const #0s : i
        let s_12_4: i128 = 0;
        // D s_12_5: read-var address:u32
        let s_12_5: u32 = fn_state.address;
        // D s_12_6: cast zx s_12_5 -> bv
        let s_12_6: Bits = Bits::new(s_12_5 as u128, 32u16);
        // C s_12_7: const #1u : u64
        let s_12_7: u64 = 1;
        // D s_12_8: bit-insert s_12_6 s_12_6 s_12_4 s_12_7
        let s_12_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_12_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_12_6.length(),
            );
            (s_12_6 & mask) | (s_12_6 << s_12_4)
        };
        // D s_12_9: cast reint s_12_8 -> u32
        let s_12_9: u32 = (s_12_8.value() as u32);
        // D s_12_10: write-var address <= s_12_9
        fn_state.address = s_12_9;
        // N s_12_11: jump b6
        return block_6(state, tracer, fn_state);
    }
}
