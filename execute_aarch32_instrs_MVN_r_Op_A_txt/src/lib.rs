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
use Shift_C::*;
use ALUExceptionReturn::*;
use R_read::*;
use R_set::*;
use IsZeroBit::*;
use ALUWritePC::*;
use common::*;
pub fn execute_aarch32_instrs_MVN_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    setflags: bool,
    shift_n: i128,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        carry: bool,
        result: u32,
        gs_881605: ProductTypef506aa96a892fe52,
        d: i64,
        m: i64,
        setflags: bool,
        shift_n: i128,
        shift_t: u32,
    }
    let fn_state = FunctionState {
        d,
        m,
        setflags,
        shift_n,
        shift_t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #16971u : u32
        let s_0_3: u32 = 16971;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: bool = {
            let value = state.read_register::<bool>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_6: read-var shift_t:u32
        let s_0_6: u32 = fn_state.shift_t;
        // D s_0_7: read-var shift_n:i
        let s_0_7: i128 = fn_state.shift_n;
        // D s_0_8: call Shift_C(s_0_5, s_0_6, s_0_7, s_0_4)
        let s_0_8: ProductTypef506aa96a892fe52 = Shift_C(
            state,
            tracer,
            s_0_5,
            s_0_6,
            s_0_7,
            s_0_4,
        );
        // D s_0_9: write-var gs#881605 <= s_0_8
        fn_state.gs_881605 = s_0_8;
        // D s_0_10: read-var gs#881605.0:struct
        let s_0_10: Bits = fn_state.gs_881605._0;
        // D s_0_11: cast reint s_0_10 -> u32
        let s_0_11: u32 = (s_0_10.value() as u32);
        // D s_0_12: read-var gs#881605.1:struct
        let s_0_12: bool = fn_state.gs_881605._1;
        // D s_0_13: write-var carry <= s_0_12
        fn_state.carry = s_0_12;
        // D s_0_14: cast zx s_0_11 -> bv
        let s_0_14: Bits = Bits::new(s_0_11 as u128, 32u16);
        // D s_0_15: not s_0_14
        let s_0_15: Bits = !s_0_14;
        // D s_0_16: cast reint s_0_15 -> u32
        let s_0_16: u32 = (s_0_15.value() as u32);
        // D s_0_17: write-var result <= s_0_16
        fn_state.result = s_0_16;
        // C s_0_18: const #15s : i
        let s_0_18: i128 = 15;
        // D s_0_19: read-var d:i64
        let s_0_19: i64 = fn_state.d;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: cmp-eq s_0_20 s_0_18
        let s_0_21: bool = ((s_0_20) == (s_0_18));
        // N s_0_22: branch s_0_21 b4 b1
        if s_0_21 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var d:i64
        let s_1_0: i64 = fn_state.d;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var result:u32
        let s_1_2: u32 = fn_state.result;
        // D s_1_3: call R_set(s_1_1, s_1_2)
        let s_1_3: () = R_set(state, tracer, s_1_1, s_1_2);
        // D s_1_4: read-var setflags:u8
        let s_1_4: bool = fn_state.setflags;
        // N s_1_5: branch s_1_4 b3 b2
        if s_1_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #31s : i
        let s_3_0: i128 = 31;
        // D s_3_1: read-var result:u32
        let s_3_1: u32 = fn_state.result;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 32u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // C s_3_18: const #16984u : u32
        let s_3_18: u32 = 16984;
        // N s_3_19: write-reg s_3_18 <= s_3_17
        let s_3_19: () = {
            state.write_register::<bool>(s_3_18 as isize, s_3_17);
            tracer.write_register(s_3_18 as isize, s_3_17);
        };
        // D s_3_20: read-var result:u32
        let s_3_20: u32 = fn_state.result;
        // D s_3_21: cast zx s_3_20 -> bv
        let s_3_21: Bits = Bits::new(s_3_20 as u128, 32u16);
        // D s_3_22: call IsZeroBit(s_3_21)
        let s_3_22: bool = IsZeroBit(state, tracer, s_3_21);
        // C s_3_23: const #16997u : u32
        let s_3_23: u32 = 16997;
        // N s_3_24: write-reg s_3_23 <= s_3_22
        let s_3_24: () = {
            state.write_register::<bool>(s_3_23 as isize, s_3_22);
            tracer.write_register(s_3_23 as isize, s_3_22);
        };
        // D s_3_25: read-var carry:u8
        let s_3_25: bool = fn_state.carry;
        // C s_3_26: const #16971u : u32
        let s_3_26: u32 = 16971;
        // N s_3_27: write-reg s_3_26 <= s_3_25
        let s_3_27: () = {
            state.write_register::<bool>(s_3_26 as isize, s_3_25);
            tracer.write_register(s_3_26 as isize, s_3_25);
        };
        // N s_3_28: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var setflags:u8
        let s_4_0: bool = fn_state.setflags;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var result:u32
        let s_5_0: u32 = fn_state.result;
        // D s_5_1: call ALUWritePC(s_5_0)
        let s_5_1: () = ALUWritePC(state, tracer, s_5_0);
        // N s_5_2: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var result:u32
        let s_6_0: u32 = fn_state.result;
        // D s_6_1: call ALUExceptionReturn(s_6_0)
        let s_6_1: () = ALUExceptionReturn(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
}
