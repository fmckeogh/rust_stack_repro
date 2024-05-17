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
use BigEndian::*;
use AArch32_SetLSInstructionSyndrome::*;
use MemO_set::*;
use R_read::*;
use AArch32_ExclusiveMonitorsPass::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_STLEXD_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
    t: i64,
    t2: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_365554: bool,
        address: u32,
        value_name: u64,
        d: i64,
        n: i64,
        t: i64,
        t2: i64,
    }
    let fn_state = FunctionState {
        d,
        n,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: write-var address <= s_0_2
        fn_state.address = s_0_2;
        // C s_0_4: const #1u : u32
        let s_0_4: u32 = 1;
        // S s_0_5: call BigEndian(s_0_4)
        let s_0_5: bool = BigEndian(state, tracer, s_0_4);
        // N s_0_6: branch s_0_5 b7 b1
        if s_0_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var t2:i64
        let s_1_0: i64 = fn_state.t2;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: read-var t:i64
        let s_1_3: i64 = fn_state.t;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call R_read(s_1_4)
        let s_1_5: u32 = R_read(state, tracer, s_1_4);
        // D s_1_6: cast zx s_1_2 -> bv
        let s_1_6: Bits = Bits::new(s_1_2 as u128, 32u16);
        // D s_1_7: cast zx s_1_5 -> bv
        let s_1_7: Bits = Bits::new(s_1_5 as u128, 32u16);
        // D s_1_8: cast reint s_1_6 -> u128
        let s_1_8: u128 = (s_1_6.value() as u128);
        // D s_1_9: size-of s_1_6
        let s_1_9: u16 = s_1_6.length();
        // D s_1_10: cast reint s_1_7 -> u128
        let s_1_10: u128 = (s_1_7.value() as u128);
        // D s_1_11: size-of s_1_7
        let s_1_11: u16 = s_1_7.length();
        // D s_1_12: lsl s_1_8 s_1_11
        let s_1_12: u128 = s_1_8 << s_1_11;
        // D s_1_13: or s_1_12 s_1_10
        let s_1_13: u128 = ((s_1_12) | (s_1_10));
        // D s_1_14: add s_1_9 s_1_11
        let s_1_14: u16 = (s_1_9 + s_1_11);
        // D s_1_15: create-bits s_1_13 s_1_14
        let s_1_15: Bits = Bits::new(s_1_13, s_1_14);
        // D s_1_16: cast reint s_1_15 -> u64
        let s_1_16: u64 = (s_1_15.value() as u64);
        // D s_1_17: write-var value_name <= s_1_16
        fn_state.value_name = s_1_16;
        // N s_1_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #8s : i
        let s_2_0: i128 = 8;
        // D s_2_1: read-var address:u32
        let s_2_1: u32 = fn_state.address;
        // D s_2_2: call AArch32_ExclusiveMonitorsPass(s_2_1, s_2_0)
        let s_2_2: bool = AArch32_ExclusiveMonitorsPass(state, tracer, s_2_1, s_2_0);
        // D s_2_3: write-var ga#365554 <= s_2_2
        fn_state.ga_365554 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#365554:u8
        let s_3_0: bool = fn_state.ga_365554;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #32s : i
        let s_4_0: i128 = 32;
        // C s_4_1: const #1u : u8
        let s_4_1: bool = true;
        // C s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 1u16);
        // D s_4_3: bits-cast zx s_4_2 -> bv length s_4_0
        let s_4_3: Bits = s_4_2.zero_extend(s_4_0);
        // D s_4_4: cast reint s_4_3 -> u32
        let s_4_4: u32 = (s_4_3.value() as u32);
        // D s_4_5: read-var d:i64
        let s_4_5: i64 = fn_state.d;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: call R_set(s_4_6, s_4_4)
        let s_4_7: () = R_set(state, tracer, s_4_6, s_4_4);
        // N s_4_8: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #8s : i
        let s_5_0: i128 = 8;
        // D s_5_1: read-var t:i64
        let s_5_1: i64 = fn_state.t;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // C s_5_3: const #0u : u8
        let s_5_3: bool = false;
        // C s_5_4: const #1u : u8
        let s_5_4: bool = true;
        // D s_5_5: call AArch32_SetLSInstructionSyndrome(s_5_0, s_5_3, s_5_2, s_5_4)
        let s_5_5: () = AArch32_SetLSInstructionSyndrome(
            state,
            tracer,
            s_5_0,
            s_5_3,
            s_5_2,
            s_5_4,
        );
        // C s_5_6: const #8s : i
        let s_5_6: i128 = 8;
        // D s_5_7: read-var value_name:u64
        let s_5_7: u64 = fn_state.value_name;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_9: read-var address:u32
        let s_5_9: u32 = fn_state.address;
        // D s_5_10: call MemO_set(s_5_9, s_5_6, s_5_8)
        let s_5_10: () = MemO_set(state, tracer, s_5_9, s_5_6, s_5_8);
        // N s_5_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #32s : i
        let s_6_0: i128 = 32;
        // C s_6_1: const #0u : u8
        let s_6_1: bool = false;
        // C s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 1u16);
        // D s_6_3: bits-cast zx s_6_2 -> bv length s_6_0
        let s_6_3: Bits = s_6_2.zero_extend(s_6_0);
        // D s_6_4: cast reint s_6_3 -> u32
        let s_6_4: u32 = (s_6_3.value() as u32);
        // D s_6_5: read-var d:i64
        let s_6_5: i64 = fn_state.d;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: call R_set(s_6_6, s_6_4)
        let s_6_7: () = R_set(state, tracer, s_6_6, s_6_4);
        // N s_6_8: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var t:i64
        let s_7_0: i64 = fn_state.t;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call R_read(s_7_1)
        let s_7_2: u32 = R_read(state, tracer, s_7_1);
        // D s_7_3: read-var t2:i64
        let s_7_3: i64 = fn_state.t2;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: call R_read(s_7_4)
        let s_7_5: u32 = R_read(state, tracer, s_7_4);
        // D s_7_6: cast zx s_7_2 -> bv
        let s_7_6: Bits = Bits::new(s_7_2 as u128, 32u16);
        // D s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 32u16);
        // D s_7_8: cast reint s_7_6 -> u128
        let s_7_8: u128 = (s_7_6.value() as u128);
        // D s_7_9: size-of s_7_6
        let s_7_9: u16 = s_7_6.length();
        // D s_7_10: cast reint s_7_7 -> u128
        let s_7_10: u128 = (s_7_7.value() as u128);
        // D s_7_11: size-of s_7_7
        let s_7_11: u16 = s_7_7.length();
        // D s_7_12: lsl s_7_8 s_7_11
        let s_7_12: u128 = s_7_8 << s_7_11;
        // D s_7_13: or s_7_12 s_7_10
        let s_7_13: u128 = ((s_7_12) | (s_7_10));
        // D s_7_14: add s_7_9 s_7_11
        let s_7_14: u16 = (s_7_9 + s_7_11);
        // D s_7_15: create-bits s_7_13 s_7_14
        let s_7_15: Bits = Bits::new(s_7_13, s_7_14);
        // D s_7_16: cast reint s_7_15 -> u64
        let s_7_16: u64 = (s_7_15.value() as u64);
        // D s_7_17: write-var value_name <= s_7_16
        fn_state.value_name = s_7_16;
        // N s_7_18: jump b2
        return block_2(state, tracer, fn_state);
    }
}
