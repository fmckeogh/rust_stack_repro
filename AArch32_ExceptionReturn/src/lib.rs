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
use SetPSTATEFromPSR::*;
use AArch32_CurrentCond::*;
use u__UNKNOWN_bits::*;
use ClearExclusiveLocal::*;
use CheckExceptionCatch::*;
use ProcessorID::*;
use Bit::*;
use SynchronizeContext::*;
use SendEventLocal::*;
use BranchTo::*;
use common::*;
pub fn AArch32_ExceptionReturn<T: Tracer>(
    state: &mut State,
    tracer: &T,
    new_pc_in: u32,
    spsr: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        new_pc: u32,
        gs_31378: bool,
        new_pc_in: u32,
        spsr: u32,
    }
    let fn_state = FunctionState {
        new_pc_in,
        spsr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var new_pc_in:u32
        let s_0_0: u32 = fn_state.new_pc_in;
        // D s_0_1: write-var new_pc <= s_0_0
        fn_state.new_pc = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call SynchronizeContext(s_0_2)
        let s_0_3: () = SynchronizeContext(state, tracer, s_0_2);
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: const #22760u : u32
        let s_0_5: u32 = 22760;
        // N s_0_6: write-reg s_0_5 <= s_0_4
        let s_0_6: () = {
            state.write_register::<bool>(s_0_5 as isize, s_0_4);
            tracer.write_register(s_0_5 as isize, s_0_4);
        };
        // D s_0_7: read-var spsr:u32
        let s_0_7: u32 = fn_state.spsr;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 32u16);
        // D s_0_9: call SetPSTATEFromPSR(s_0_8)
        let s_0_9: () = SetPSTATEFromPSR(state, tracer, s_0_8);
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call ProcessorID(s_0_10)
        let s_0_11: i128 = ProcessorID(state, tracer, s_0_10);
        // S s_0_12: call ClearExclusiveLocal(s_0_11)
        let s_0_12: () = ClearExclusiveLocal(state, tracer, s_0_11);
        // C s_0_13: const #() : ()
        let s_0_13: () = ();
        // S s_0_14: call SendEventLocal(s_0_13)
        let s_0_14: () = SendEventLocal(state, tracer, s_0_13);
        // C s_0_15: const #16980u : u32
        let s_0_15: u32 = 16980;
        // D s_0_16: read-reg s_0_15:u8
        let s_0_16: bool = {
            let value = state.read_register::<bool>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 1u16);
        // C s_0_18: const #1u : u8
        let s_0_18: bool = true;
        // C s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // D s_0_20: cmp-eq s_0_17 s_0_19
        let s_0_20: bool = ((s_0_17) == (s_0_19));
        // N s_0_21: branch s_0_20 b8 b1
        if s_0_20 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16993u : u32
        let s_1_0: u32 = 16993;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: bool = {
            let value = state.read_register::<bool>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 1u16);
        // C s_1_3: const #1u : u8
        let s_1_3: bool = true;
        // C s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 1u16);
        // D s_1_5: cmp-eq s_1_2 s_1_4
        let s_1_5: bool = ((s_1_2) == (s_1_4));
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
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
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var new_pc:u32
        let s_2_1: u32 = fn_state.new_pc;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // C s_2_3: const #0u : u8
        let s_2_3: u8 = 0;
        // C s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // C s_2_5: const #1s : i
        let s_2_5: i128 = 1;
        // C s_2_6: const #1u : u64
        let s_2_6: u64 = 1;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 64u16);
        // C s_2_8: lsl s_2_7 s_2_5
        let s_2_8: Bits = s_2_7 << s_2_5;
        // C s_2_9: sub s_2_8 s_2_7
        let s_2_9: Bits = ((s_2_8) - (s_2_7));
        // C s_2_10: and s_2_4 s_2_9
        let s_2_10: Bits = ((s_2_4) & (s_2_9));
        // C s_2_11: lsl s_2_10 s_2_0
        let s_2_11: Bits = s_2_10 << s_2_0;
        // C s_2_12: lsl s_2_9 s_2_0
        let s_2_12: Bits = s_2_9 << s_2_0;
        // C s_2_13: cmpl s_2_12
        let s_2_13: Bits = !s_2_12;
        // D s_2_14: and s_2_2 s_2_13
        let s_2_14: Bits = ((s_2_2) & (s_2_13));
        // D s_2_15: or s_2_14 s_2_11
        let s_2_15: Bits = ((s_2_14) | (s_2_11));
        // D s_2_16: cast reint s_2_15 -> u32
        let s_2_16: u32 = (s_2_15.value() as u32);
        // D s_2_17: write-var new_pc <= s_2_16
        fn_state.new_pc = s_2_16;
        // N s_2_18: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call AArch32_CurrentCond(s_3_0)
        let s_3_1: u8 = AArch32_CurrentCond(state, tracer, s_3_0);
        // C s_3_2: const #1s : i
        let s_3_2: i128 = 1;
        // S s_3_3: cast zx s_3_1 -> bv
        let s_3_3: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_4: const #1s : i64
        let s_3_4: i64 = 1;
        // C s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // C s_3_6: const #2s : i
        let s_3_6: i128 = 2;
        // C s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: bit-extract s_3_3 s_3_2 s_3_7
        let s_3_8: Bits = (Bits::new(
            ((s_3_3) >> (s_3_2)).value(),
            u16::try_from(s_3_7).unwrap(),
        ));
        // D s_3_9: cast reint s_3_8 -> u8
        let s_3_9: u8 = (s_3_8.value() as u8);
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 3u16);
        // C s_3_11: const #7u : u8
        let s_3_11: u8 = 7;
        // C s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 3u16);
        // D s_3_13: cmp-eq s_3_10 s_3_12
        let s_3_13: bool = ((s_3_10) == (s_3_12));
        // D s_3_14: not s_3_13
        let s_3_14: bool = !s_3_13;
        // N s_3_15: branch s_3_14 b6 b4
        if s_3_14 {
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
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // D s_4_1: write-var gs#31378 <= s_4_0
        fn_state.gs_31378 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#31378:u8
        let s_5_0: bool = fn_state.gs_31378;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // D s_5_2: read-var new_pc:u32
        let s_5_2: u32 = fn_state.new_pc;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 32u16);
        // C s_5_4: const #2u : u32
        let s_5_4: u32 = 2;
        // D s_5_5: call BranchTo(s_5_3, s_5_4, s_5_1)
        let s_5_5: () = BranchTo(state, tracer, s_5_3, s_5_4, s_5_1);
        // C s_5_6: const #0u : u8
        let s_5_6: bool = false;
        // S s_5_7: call CheckExceptionCatch(s_5_6)
        let s_5_7: () = CheckExceptionCatch(state, tracer, s_5_6);
        // N s_5_8: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#31378 <= s_6_0
        fn_state.gs_31378 = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // S s_7_1: call Bit(s_7_0)
        let s_7_1: bool = Bit(state, tracer, s_7_0);
        // C s_7_2: const #0s : i
        let s_7_2: i128 = 0;
        // D s_7_3: read-var new_pc:u32
        let s_7_3: u32 = fn_state.new_pc;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 32u16);
        // C s_7_5: const #1u : u64
        let s_7_5: u64 = 1;
        // D s_7_6: bit-insert s_7_4 s_7_4 s_7_2 s_7_5
        let s_7_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_7_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_7_4.length(),
            );
            (s_7_4 & mask) | (s_7_4 << s_7_2)
        };
        // D s_7_7: cast reint s_7_6 -> u32
        let s_7_7: u32 = (s_7_6.value() as u32);
        // D s_7_8: write-var new_pc <= s_7_7
        fn_state.new_pc = s_7_7;
        // N s_7_9: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2s : i64
        let s_8_0: i64 = 2;
        // C s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // S s_8_2: call __UNKNOWN_bits(s_8_1)
        let s_8_2: Bits = u__UNKNOWN_bits(state, tracer, s_8_1);
        // S s_8_3: cast reint s_8_2 -> u8
        let s_8_3: u8 = (s_8_2.value() as u8);
        // C s_8_4: const #0s : i
        let s_8_4: i128 = 0;
        // D s_8_5: read-var new_pc:u32
        let s_8_5: u32 = fn_state.new_pc;
        // D s_8_6: cast zx s_8_5 -> bv
        let s_8_6: Bits = Bits::new(s_8_5 as u128, 32u16);
        // S s_8_7: cast zx s_8_3 -> bv
        let s_8_7: Bits = Bits::new(s_8_3 as u128, 2u16);
        // C s_8_8: const #1s : i
        let s_8_8: i128 = 1;
        // C s_8_9: const #1u : u64
        let s_8_9: u64 = 1;
        // C s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 64u16);
        // C s_8_11: lsl s_8_10 s_8_8
        let s_8_11: Bits = s_8_10 << s_8_8;
        // C s_8_12: sub s_8_11 s_8_10
        let s_8_12: Bits = ((s_8_11) - (s_8_10));
        // S s_8_13: and s_8_7 s_8_12
        let s_8_13: Bits = ((s_8_7) & (s_8_12));
        // S s_8_14: lsl s_8_13 s_8_4
        let s_8_14: Bits = s_8_13 << s_8_4;
        // C s_8_15: lsl s_8_12 s_8_4
        let s_8_15: Bits = s_8_12 << s_8_4;
        // C s_8_16: cmpl s_8_15
        let s_8_16: Bits = !s_8_15;
        // D s_8_17: and s_8_6 s_8_16
        let s_8_17: Bits = ((s_8_6) & (s_8_16));
        // D s_8_18: or s_8_17 s_8_14
        let s_8_18: Bits = ((s_8_17) | (s_8_14));
        // D s_8_19: cast reint s_8_18 -> u32
        let s_8_19: u32 = (s_8_18.value() as u32);
        // D s_8_20: write-var new_pc <= s_8_19
        fn_state.new_pc = s_8_19;
        // N s_8_21: jump b3
        return block_3(state, tracer, fn_state);
    }
}
