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
use MemA_read::*;
use R_read::*;
use R_set::*;
use AArch32_SetExclusiveMonitors::*;
use common::*;
pub fn execute_aarch32_instrs_LDREXD_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    t: i64,
    t2: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_879951: Bits,
        ga_344235: u32,
        address: u32,
        value_name: u64,
        ga_344233: u32,
        n: i64,
        t: i64,
        t2: i64,
    }
    let fn_state = FunctionState {
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
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: read-var address:u32
        let s_0_5: u32 = fn_state.address;
        // D s_0_6: call AArch32_SetExclusiveMonitors(s_0_5, s_0_4)
        let s_0_6: () = AArch32_SetExclusiveMonitors(state, tracer, s_0_5, s_0_4);
        // N s_0_7: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #8s : i64
        let s_1_0: i64 = 8;
        // D s_1_1: read-var address:u32
        let s_1_1: u32 = fn_state.address;
        // D s_1_2: call MemA_read(s_1_1, s_1_0)
        let s_1_2: Bits = MemA_read(state, tracer, s_1_1, s_1_0);
        // D s_1_3: write-var gs#879951 <= s_1_2
        fn_state.gs_879951 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#879951:bv
        let s_2_0: Bits = fn_state.gs_879951;
        // D s_2_1: cast reint s_2_0 -> u64
        let s_2_1: u64 = (s_2_0.value() as u64);
        // D s_2_2: write-var value_name <= s_2_1
        fn_state.value_name = s_2_1;
        // C s_2_3: const #1u : u32
        let s_2_3: u32 = 1;
        // S s_2_4: call BigEndian(s_2_3)
        let s_2_4: bool = BigEndian(state, tracer, s_2_3);
        // N s_2_5: branch s_2_4 b8 b3
        if s_2_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var value_name:u64
        let s_3_1: u64 = fn_state.value_name;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 64u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #31s : i
        let s_3_5: i128 = 31;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u32
        let s_3_8: u32 = (s_3_7.value() as u32);
        // D s_3_9: write-var ga#344233 <= s_3_8
        fn_state.ga_344233 = s_3_8;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var t:i64
        let s_4_0: i64 = fn_state.t;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var ga#344233:u32
        let s_4_2: u32 = fn_state.ga_344233;
        // D s_4_3: call R_set(s_4_1, s_4_2)
        let s_4_3: () = R_set(state, tracer, s_4_1, s_4_2);
        // C s_4_4: const #1u : u32
        let s_4_4: u32 = 1;
        // S s_4_5: call BigEndian(s_4_4)
        let s_4_5: bool = BigEndian(state, tracer, s_4_4);
        // N s_4_6: branch s_4_5 b7 b5
        if s_4_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #32s : i
        let s_5_0: i128 = 32;
        // D s_5_1: read-var value_name:u64
        let s_5_1: u64 = fn_state.value_name;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 64u16);
        // C s_5_3: const #1s : i64
        let s_5_3: i64 = 1;
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: const #31s : i
        let s_5_5: i128 = 31;
        // C s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // D s_5_7: bit-extract s_5_2 s_5_0 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u32
        let s_5_8: u32 = (s_5_7.value() as u32);
        // D s_5_9: write-var ga#344235 <= s_5_8
        fn_state.ga_344235 = s_5_8;
        // N s_5_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var t2:i64
        let s_6_0: i64 = fn_state.t2;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var ga#344235:u32
        let s_6_2: u32 = fn_state.ga_344235;
        // D s_6_3: call R_set(s_6_1, s_6_2)
        let s_6_3: () = R_set(state, tracer, s_6_1, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var value_name:u64
        let s_7_1: u64 = fn_state.value_name;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #31s : i
        let s_7_5: i128 = 31;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_0 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u32
        let s_7_8: u32 = (s_7_7.value() as u32);
        // D s_7_9: write-var ga#344235 <= s_7_8
        fn_state.ga_344235 = s_7_8;
        // N s_7_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #32s : i
        let s_8_0: i128 = 32;
        // D s_8_1: read-var value_name:u64
        let s_8_1: u64 = fn_state.value_name;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 64u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #31s : i
        let s_8_5: i128 = 31;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_0 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u32
        let s_8_8: u32 = (s_8_7.value() as u32);
        // D s_8_9: write-var ga#344233 <= s_8_8
        fn_state.ga_344233 = s_8_8;
        // N s_8_10: jump b4
        return block_4(state, tracer, fn_state);
    }
}
