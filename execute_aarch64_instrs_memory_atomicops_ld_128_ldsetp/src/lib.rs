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
use MemAtomic::*;
use CreateAccDescAtomicOp::*;
use u__UNKNOWN_bits::*;
use X_set::*;
use SP_read::*;
use BigEndian::*;
use X_read::*;
use CheckSPAlignment::*;
use common::*;
pub fn execute_aarch64_instrs_memory_atomicops_ld_128_ldsetp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acquire: bool,
    n: i64,
    op: u32,
    release: bool,
    t: i64,
    t2: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        value2_name: u64,
        data: u128,
        value1_name: u64,
        address: u64,
        gs_705920: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        store_value: u128,
        acquire: bool,
        n: i64,
        op: u32,
        release: bool,
        t: i64,
        t2: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        acquire,
        n,
        op,
        release,
        t,
        t2,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // D s_0_1: read-var t:i64
        let s_0_1: i64 = fn_state.t;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: call X_read(s_0_2, s_0_0)
        let s_0_3: Bits = X_read(state, tracer, s_0_2, s_0_0);
        // D s_0_4: cast reint s_0_3 -> u64
        let s_0_4: u64 = (s_0_3.value() as u64);
        // D s_0_5: write-var value1_name <= s_0_4
        fn_state.value1_name = s_0_4;
        // C s_0_6: const #64s : i64
        let s_0_6: i64 = 64;
        // D s_0_7: read-var t2:i64
        let s_0_7: i64 = fn_state.t2;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: call X_read(s_0_8, s_0_6)
        let s_0_9: Bits = X_read(state, tracer, s_0_8, s_0_6);
        // D s_0_10: cast reint s_0_9 -> u64
        let s_0_10: u64 = (s_0_9.value() as u64);
        // D s_0_11: write-var value2_name <= s_0_10
        fn_state.value2_name = s_0_10;
        // D s_0_12: read-var op:u32
        let s_0_12: u32 = fn_state.op;
        // D s_0_13: read-var acquire:u8
        let s_0_13: bool = fn_state.acquire;
        // D s_0_14: read-var release:u8
        let s_0_14: bool = fn_state.release;
        // D s_0_15: read-var tagchecked:u8
        let s_0_15: bool = fn_state.tagchecked;
        // D s_0_16: call CreateAccDescAtomicOp(s_0_12, s_0_13, s_0_14, s_0_15)
        let s_0_16: ProductType9878976b5bcce9c9 = CreateAccDescAtomicOp(
            state,
            tracer,
            s_0_12,
            s_0_13,
            s_0_14,
            s_0_15,
        );
        // D s_0_17: write-var accdesc <= s_0_16
        fn_state.accdesc = s_0_16;
        // C s_0_18: const #31s : i
        let s_0_18: i128 = 31;
        // D s_0_19: read-var n:i64
        let s_0_19: i64 = fn_state.n;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: cmp-eq s_0_20 s_0_18
        let s_0_21: bool = ((s_0_20) == (s_0_18));
        // N s_0_22: branch s_0_21 b9 b1
        if s_0_21 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var address <= s_1_4
        fn_state.address = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var accdesc.1:struct
        let s_2_0: u32 = fn_state.accdesc._1;
        // D s_2_1: call BigEndian(s_2_0)
        let s_2_1: bool = BigEndian(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b8 b3
        if s_2_1 {
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
        // D s_3_0: read-var value2_name:u64
        let s_3_0: u64 = fn_state.value2_name;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 64u16);
        // D s_3_2: read-var value1_name:u64
        let s_3_2: u64 = fn_state.value1_name;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 64u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u128
        let s_3_12: u128 = (s_3_11.value() as u128);
        // D s_3_13: write-var store_value <= s_3_12
        fn_state.store_value = s_3_12;
        // N s_3_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // C s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // S s_4_2: call __UNKNOWN_bits(s_4_1)
        let s_4_2: Bits = u__UNKNOWN_bits(state, tracer, s_4_1);
        // S s_4_3: cast reint s_4_2 -> u128
        let s_4_3: u128 = (s_4_2.value() as u128);
        // S s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 128u16);
        // D s_4_5: read-var store_value:u128
        let s_4_5: u128 = fn_state.store_value;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 128u16);
        // D s_4_7: read-var address:u64
        let s_4_7: u64 = fn_state.address;
        // D s_4_8: read-var accdesc:struct
        let s_4_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_9: call MemAtomic(s_4_7, s_4_4, s_4_6, s_4_8)
        let s_4_9: Bits = MemAtomic(state, tracer, s_4_7, s_4_4, s_4_6, s_4_8);
        // D s_4_10: write-var gs#705920 <= s_4_9
        fn_state.gs_705920 = s_4_9;
        // N s_4_11: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#705920:bv
        let s_5_0: Bits = fn_state.gs_705920;
        // D s_5_1: cast reint s_5_0 -> u128
        let s_5_1: u128 = (s_5_0.value() as u128);
        // D s_5_2: write-var data <= s_5_1
        fn_state.data = s_5_1;
        // D s_5_3: read-var accdesc.1:struct
        let s_5_3: u32 = fn_state.accdesc._1;
        // D s_5_4: call BigEndian(s_5_3)
        let s_5_4: bool = BigEndian(state, tracer, s_5_3);
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
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
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #0s : i
        let s_6_1: i128 = 0;
        // D s_6_2: read-var data:u128
        let s_6_2: u128 = fn_state.data;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 128u16);
        // C s_6_4: const #1s : i64
        let s_6_4: i64 = 1;
        // C s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // C s_6_6: const #63s : i
        let s_6_6: i128 = 63;
        // C s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: bit-extract s_6_3 s_6_1 s_6_7
        let s_6_8: Bits = (Bits::new(
            ((s_6_3) >> (s_6_1)).value(),
            u16::try_from(s_6_7).unwrap(),
        ));
        // D s_6_9: cast reint s_6_8 -> u64
        let s_6_9: u64 = (s_6_8.value() as u64);
        // D s_6_10: read-var t:i64
        let s_6_10: i64 = fn_state.t;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: cast zx s_6_9 -> bv
        let s_6_12: Bits = Bits::new(s_6_9 as u128, 64u16);
        // D s_6_13: call X_set(s_6_11, s_6_0, s_6_12)
        let s_6_13: () = X_set(state, tracer, s_6_11, s_6_0, s_6_12);
        // C s_6_14: const #64s : i64
        let s_6_14: i64 = 64;
        // C s_6_15: const #64s : i
        let s_6_15: i128 = 64;
        // D s_6_16: read-var data:u128
        let s_6_16: u128 = fn_state.data;
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 128u16);
        // C s_6_18: const #1s : i64
        let s_6_18: i64 = 1;
        // C s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // C s_6_20: const #63s : i
        let s_6_20: i128 = 63;
        // C s_6_21: add s_6_20 s_6_19
        let s_6_21: i128 = (s_6_20 + s_6_19);
        // D s_6_22: bit-extract s_6_17 s_6_15 s_6_21
        let s_6_22: Bits = (Bits::new(
            ((s_6_17) >> (s_6_15)).value(),
            u16::try_from(s_6_21).unwrap(),
        ));
        // D s_6_23: cast reint s_6_22 -> u64
        let s_6_23: u64 = (s_6_22.value() as u64);
        // D s_6_24: read-var t2:i64
        let s_6_24: i64 = fn_state.t2;
        // D s_6_25: cast zx s_6_24 -> i
        let s_6_25: i128 = (i128::try_from(s_6_24).unwrap());
        // D s_6_26: cast zx s_6_23 -> bv
        let s_6_26: Bits = Bits::new(s_6_23 as u128, 64u16);
        // D s_6_27: call X_set(s_6_25, s_6_14, s_6_26)
        let s_6_27: () = X_set(state, tracer, s_6_25, s_6_14, s_6_26);
        // N s_6_28: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #64s : i
        let s_7_1: i128 = 64;
        // D s_7_2: read-var data:u128
        let s_7_2: u128 = fn_state.data;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 128u16);
        // C s_7_4: const #1s : i64
        let s_7_4: i64 = 1;
        // C s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // C s_7_6: const #63s : i
        let s_7_6: i128 = 63;
        // C s_7_7: add s_7_6 s_7_5
        let s_7_7: i128 = (s_7_6 + s_7_5);
        // D s_7_8: bit-extract s_7_3 s_7_1 s_7_7
        let s_7_8: Bits = (Bits::new(
            ((s_7_3) >> (s_7_1)).value(),
            u16::try_from(s_7_7).unwrap(),
        ));
        // D s_7_9: cast reint s_7_8 -> u64
        let s_7_9: u64 = (s_7_8.value() as u64);
        // D s_7_10: read-var t:i64
        let s_7_10: i64 = fn_state.t;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: cast zx s_7_9 -> bv
        let s_7_12: Bits = Bits::new(s_7_9 as u128, 64u16);
        // D s_7_13: call X_set(s_7_11, s_7_0, s_7_12)
        let s_7_13: () = X_set(state, tracer, s_7_11, s_7_0, s_7_12);
        // C s_7_14: const #64s : i64
        let s_7_14: i64 = 64;
        // C s_7_15: const #0s : i
        let s_7_15: i128 = 0;
        // D s_7_16: read-var data:u128
        let s_7_16: u128 = fn_state.data;
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 128u16);
        // C s_7_18: const #1s : i64
        let s_7_18: i64 = 1;
        // C s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // C s_7_20: const #63s : i
        let s_7_20: i128 = 63;
        // C s_7_21: add s_7_20 s_7_19
        let s_7_21: i128 = (s_7_20 + s_7_19);
        // D s_7_22: bit-extract s_7_17 s_7_15 s_7_21
        let s_7_22: Bits = (Bits::new(
            ((s_7_17) >> (s_7_15)).value(),
            u16::try_from(s_7_21).unwrap(),
        ));
        // D s_7_23: cast reint s_7_22 -> u64
        let s_7_23: u64 = (s_7_22.value() as u64);
        // D s_7_24: read-var t2:i64
        let s_7_24: i64 = fn_state.t2;
        // D s_7_25: cast zx s_7_24 -> i
        let s_7_25: i128 = (i128::try_from(s_7_24).unwrap());
        // D s_7_26: cast zx s_7_23 -> bv
        let s_7_26: Bits = Bits::new(s_7_23 as u128, 64u16);
        // D s_7_27: call X_set(s_7_25, s_7_14, s_7_26)
        let s_7_27: () = X_set(state, tracer, s_7_25, s_7_14, s_7_26);
        // N s_7_28: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var value1_name:u64
        let s_8_0: u64 = fn_state.value1_name;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 64u16);
        // D s_8_2: read-var value2_name:u64
        let s_8_2: u64 = fn_state.value2_name;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 64u16);
        // D s_8_4: cast reint s_8_1 -> u128
        let s_8_4: u128 = (s_8_1.value() as u128);
        // D s_8_5: size-of s_8_1
        let s_8_5: u16 = s_8_1.length();
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // D s_8_8: lsl s_8_4 s_8_7
        let s_8_8: u128 = s_8_4 << s_8_7;
        // D s_8_9: or s_8_8 s_8_6
        let s_8_9: u128 = ((s_8_8) | (s_8_6));
        // D s_8_10: add s_8_5 s_8_7
        let s_8_10: u16 = (s_8_5 + s_8_7);
        // D s_8_11: create-bits s_8_9 s_8_10
        let s_8_11: Bits = Bits::new(s_8_9, s_8_10);
        // D s_8_12: cast reint s_8_11 -> u128
        let s_8_12: u128 = (s_8_11.value() as u128);
        // D s_8_13: write-var store_value <= s_8_12
        fn_state.store_value = s_8_12;
        // N s_8_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call CheckSPAlignment(s_9_0)
        let s_9_1: () = CheckSPAlignment(state, tracer, s_9_0);
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call SP_read(s_10_0)
        let s_10_1: u64 = SP_read(state, tracer, s_10_0);
        // D s_10_2: write-var address <= s_10_1
        fn_state.address = s_10_1;
        // N s_10_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
