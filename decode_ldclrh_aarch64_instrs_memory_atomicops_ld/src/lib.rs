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
use neq_int::*;
use HaveAtomicExt::*;
use execute_aarch64_instrs_memory_atomicops_ld::*;
use common::*;
pub fn decode_ldclrh_aarch64_instrs_memory_atomicops_ld<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    opc: u8,
    Rs: u8,
    R: bool,
    A: bool,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        s: i64,
        gs_157856: bool,
        op: u32,
        n: i64,
        is_load: bool,
        release: bool,
        regsize: i64,
        ga_258461: i64,
        datasize: i64,
        acquire: bool,
        Rt: u8,
        Rn: u8,
        opc: u8,
        Rs: u8,
        R: bool,
        A: bool,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        opc,
        Rs,
        R,
        A,
        size,
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
        // S s_0_1: call HaveAtomicExt(s_0_0)
        let s_0_1: bool = HaveAtomicExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b23 b1
        if s_0_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rt:u8
        let s_1_0: u8 = fn_state.Rt;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var t <= s_1_3
        fn_state.t = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var Rs:u8
        let s_1_10: u8 = fn_state.Rs;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (s_1_11.value() as i128);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var s <= s_1_13
        fn_state.s = s_1_13;
        // D s_1_15: read-var size:u8
        let s_1_15: u8 = fn_state.size;
        // D s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 2u16);
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (s_1_16.value() as i128);
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // C s_1_19: const #8s : i64
        let s_1_19: i64 = 8;
        // D s_1_20: lsl s_1_19 s_1_18
        let s_1_20: i64 = s_1_19 << s_1_18;
        // D s_1_21: write-var datasize <= s_1_20
        fn_state.datasize = s_1_20;
        // C s_1_22: const #64s : i
        let s_1_22: i128 = 64;
        // D s_1_23: read-var datasize:i64
        let s_1_23: i64 = fn_state.datasize;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cmp-eq s_1_24 s_1_22
        let s_1_25: bool = ((s_1_24) == (s_1_22));
        // N s_1_26: branch s_1_25 b22 b2
        if s_1_25 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #32s : i64
        let s_2_0: i64 = 32;
        // D s_2_1: write-var ga#258461 <= s_2_0
        fn_state.ga_258461 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#258461:i64
        let s_3_0: i64 = fn_state.ga_258461;
        // D s_3_1: write-var regsize <= s_3_0
        fn_state.regsize = s_3_0;
        // D s_3_2: read-var A:u8
        let s_3_2: bool = fn_state.A;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // C s_3_4: const #1u : u8
        let s_3_4: bool = true;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // N s_3_7: branch s_3_6 b21 b4
        if s_3_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#157856 <= s_4_0
        fn_state.gs_157856 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#157856:u8
        let s_5_0: bool = fn_state.gs_157856;
        // D s_5_1: write-var acquire <= s_5_0
        fn_state.acquire = s_5_0;
        // D s_5_2: read-var Rt:u8
        let s_5_2: u8 = fn_state.Rt;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 5u16);
        // C s_5_4: const #31u : u8
        let s_5_4: u8 = 31;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 5u16);
        // D s_5_6: cmp-ne s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) != (s_5_5));
        // D s_5_7: write-var is_load <= s_5_6
        fn_state.is_load = s_5_6;
        // D s_5_8: read-var R:u8
        let s_5_8: bool = fn_state.R;
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 1u16);
        // C s_5_10: const #1u : u8
        let s_5_10: bool = true;
        // C s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 1u16);
        // D s_5_12: cmp-eq s_5_9 s_5_11
        let s_5_12: bool = ((s_5_9) == (s_5_11));
        // D s_5_13: write-var release <= s_5_12
        fn_state.release = s_5_12;
        // D s_5_14: read-var opc:u8
        let s_5_14: u8 = fn_state.opc;
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 3u16);
        // C s_5_16: const #0u : u8
        let s_5_16: u8 = 0;
        // C s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 3u16);
        // D s_5_18: cmp-eq s_5_15 s_5_17
        let s_5_18: bool = ((s_5_15) == (s_5_17));
        // D s_5_19: not s_5_18
        let s_5_19: bool = !s_5_18;
        // N s_5_20: branch s_5_19 b8 b6
        if s_5_19 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: write-var op <= s_6_0
        fn_state.op = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var op:u32
        let s_7_0: u32 = fn_state.op;
        // C s_7_1: const #31s : i
        let s_7_1: i128 = 31;
        // D s_7_2: read-var n:i64
        let s_7_2: i64 = fn_state.n;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: call neq_int(s_7_3, s_7_1)
        let s_7_4: bool = neq_int(state, tracer, s_7_3, s_7_1);
        // D s_7_5: read-var datasize:i64
        let s_7_5: i64 = fn_state.datasize;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: read-var regsize:i64
        let s_7_8: i64 = fn_state.regsize;
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: read-var acquire:u8
        let s_7_11: bool = fn_state.acquire;
        // D s_7_12: read-var is_load:u8
        let s_7_12: bool = fn_state.is_load;
        // D s_7_13: read-var n:i64
        let s_7_13: i64 = fn_state.n;
        // D s_7_14: read-var release:u8
        let s_7_14: bool = fn_state.release;
        // D s_7_15: read-var s:i64
        let s_7_15: i64 = fn_state.s;
        // D s_7_16: read-var t:i64
        let s_7_16: i64 = fn_state.t;
        // D s_7_17: call execute_aarch64_instrs_memory_atomicops_ld(s_7_11, s_7_7, s_7_12, s_7_13, s_7_0, s_7_10, s_7_14, s_7_15, s_7_16, s_7_4)
        let s_7_17: () = execute_aarch64_instrs_memory_atomicops_ld(
            state,
            tracer,
            s_7_11,
            s_7_7,
            s_7_12,
            s_7_13,
            s_7_0,
            s_7_10,
            s_7_14,
            s_7_15,
            s_7_16,
            s_7_4,
        );
        // N s_7_18: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var opc:u8
        let s_8_0: u8 = fn_state.opc;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 3u16);
        // C s_8_2: const #1u : u8
        let s_8_2: u8 = 1;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 3u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #2u : u32
        let s_9_0: u32 = 2;
        // D s_9_1: write-var op <= s_9_0
        fn_state.op = s_9_0;
        // N s_9_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var opc:u8
        let s_10_0: u8 = fn_state.opc;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 3u16);
        // C s_10_2: const #2u : u8
        let s_10_2: u8 = 2;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 3u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #3u : u32
        let s_11_0: u32 = 3;
        // D s_11_1: write-var op <= s_11_0
        fn_state.op = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var opc:u8
        let s_12_0: u8 = fn_state.opc;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 3u16);
        // C s_12_2: const #3u : u8
        let s_12_2: u8 = 3;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 3u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
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
        // C s_13_0: const #4u : u32
        let s_13_0: u32 = 4;
        // D s_13_1: write-var op <= s_13_0
        fn_state.op = s_13_0;
        // N s_13_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var opc:u8
        let s_14_0: u8 = fn_state.opc;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 3u16);
        // C s_14_2: const #4u : u8
        let s_14_2: u8 = 4;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 3u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b16 b15
        if s_14_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #5u : u32
        let s_15_0: u32 = 5;
        // D s_15_1: write-var op <= s_15_0
        fn_state.op = s_15_0;
        // N s_15_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var opc:u8
        let s_16_0: u8 = fn_state.opc;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 3u16);
        // C s_16_2: const #5u : u8
        let s_16_2: u8 = 5;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 3u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: not s_16_4
        let s_16_5: bool = !s_16_4;
        // N s_16_6: branch s_16_5 b18 b17
        if s_16_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #6u : u32
        let s_17_0: u32 = 6;
        // D s_17_1: write-var op <= s_17_0
        fn_state.op = s_17_0;
        // N s_17_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var opc:u8
        let s_18_0: u8 = fn_state.opc;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 3u16);
        // C s_18_2: const #6u : u8
        let s_18_2: u8 = 6;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 3u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #7u : u32
        let s_19_0: u32 = 7;
        // D s_19_1: write-var op <= s_19_0
        fn_state.op = s_19_0;
        // N s_19_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #8u : u32
        let s_20_0: u32 = 8;
        // D s_20_1: write-var op <= s_20_0
        fn_state.op = s_20_0;
        // N s_20_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var Rt:u8
        let s_21_0: u8 = fn_state.Rt;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 5u16);
        // C s_21_2: const #31u : u8
        let s_21_2: u8 = 31;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 5u16);
        // D s_21_4: cmp-ne s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) != (s_21_3));
        // D s_21_5: write-var gs#157856 <= s_21_4
        fn_state.gs_157856 = s_21_4;
        // N s_21_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #64s : i64
        let s_22_0: i64 = 64;
        // D s_22_1: write-var ga#258461 <= s_22_0
        fn_state.ga_258461 = s_22_0;
        // N s_22_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
}
