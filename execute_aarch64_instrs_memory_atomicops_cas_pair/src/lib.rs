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
use X_set::*;
use SP_read::*;
use BigEndian::*;
use X_read::*;
use CheckSPAlignment::*;
use SPESampleExtendedLoadStore::*;
use common::*;
pub fn execute_aarch64_instrs_memory_atomicops_cas_pair<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acquire: bool,
    datasize: i64,
    n: i64,
    release: bool,
    s: i64,
    t: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t2: Bits,
        address: u64,
        datasizeshadow_1112: i64,
        t1: Bits,
        comparevalue: Bits,
        ar: bool,
        gs_145808: bool,
        s2: Bits,
        data: Bits,
        newvalue: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        s1: Bits,
        acquire: bool,
        datasize: i64,
        n: i64,
        release: bool,
        s: i64,
        t: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        acquire,
        datasize,
        n,
        release,
        s,
        t,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1112 <= s_0_2
        fn_state.datasizeshadow_1112 = s_0_2;
        // D s_0_4: read-var datasizeshadow#1112:i64
        let s_0_4: i64 = fn_state.datasizeshadow_1112;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: read-var s:i64
        let s_0_7: i64 = fn_state.s;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: call X_read(s_0_8, s_0_6)
        let s_0_9: Bits = X_read(state, tracer, s_0_8, s_0_6);
        // D s_0_10: write-var s1 <= s_0_9
        fn_state.s1 = s_0_9;
        // C s_0_11: const #1s : i
        let s_0_11: i128 = 1;
        // D s_0_12: read-var s:i64
        let s_0_12: i64 = fn_state.s;
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // D s_0_14: add s_0_13 s_0_11
        let s_0_14: i128 = (s_0_13 + s_0_11);
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // D s_0_16: read-var datasizeshadow#1112:i64
        let s_0_16: i64 = fn_state.datasizeshadow_1112;
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (i128::try_from(s_0_16).unwrap());
        // D s_0_18: cast reint s_0_17 -> i64
        let s_0_18: i64 = (s_0_17 as i64);
        // D s_0_19: cast zx s_0_15 -> i
        let s_0_19: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_20: call X_read(s_0_19, s_0_18)
        let s_0_20: Bits = X_read(state, tracer, s_0_19, s_0_18);
        // D s_0_21: write-var s2 <= s_0_20
        fn_state.s2 = s_0_20;
        // D s_0_22: read-var datasizeshadow#1112:i64
        let s_0_22: i64 = fn_state.datasizeshadow_1112;
        // D s_0_23: cast zx s_0_22 -> i
        let s_0_23: i128 = (i128::try_from(s_0_22).unwrap());
        // D s_0_24: cast reint s_0_23 -> i64
        let s_0_24: i64 = (s_0_23 as i64);
        // D s_0_25: read-var t:i64
        let s_0_25: i64 = fn_state.t;
        // D s_0_26: cast zx s_0_25 -> i
        let s_0_26: i128 = (i128::try_from(s_0_25).unwrap());
        // D s_0_27: call X_read(s_0_26, s_0_24)
        let s_0_27: Bits = X_read(state, tracer, s_0_26, s_0_24);
        // D s_0_28: write-var t1 <= s_0_27
        fn_state.t1 = s_0_27;
        // C s_0_29: const #1s : i
        let s_0_29: i128 = 1;
        // D s_0_30: read-var t:i64
        let s_0_30: i64 = fn_state.t;
        // D s_0_31: cast zx s_0_30 -> i
        let s_0_31: i128 = (i128::try_from(s_0_30).unwrap());
        // D s_0_32: add s_0_31 s_0_29
        let s_0_32: i128 = (s_0_31 + s_0_29);
        // D s_0_33: cast reint s_0_32 -> i64
        let s_0_33: i64 = (s_0_32 as i64);
        // D s_0_34: read-var datasizeshadow#1112:i64
        let s_0_34: i64 = fn_state.datasizeshadow_1112;
        // D s_0_35: cast zx s_0_34 -> i
        let s_0_35: i128 = (i128::try_from(s_0_34).unwrap());
        // D s_0_36: cast reint s_0_35 -> i64
        let s_0_36: i64 = (s_0_35 as i64);
        // D s_0_37: cast zx s_0_33 -> i
        let s_0_37: i128 = (i128::try_from(s_0_33).unwrap());
        // D s_0_38: call X_read(s_0_37, s_0_36)
        let s_0_38: Bits = X_read(state, tracer, s_0_37, s_0_36);
        // D s_0_39: write-var t2 <= s_0_38
        fn_state.t2 = s_0_38;
        // C s_0_40: const #10u : u32
        let s_0_40: u32 = 10;
        // D s_0_41: read-var acquire:u8
        let s_0_41: bool = fn_state.acquire;
        // D s_0_42: read-var release:u8
        let s_0_42: bool = fn_state.release;
        // D s_0_43: read-var tagchecked:u8
        let s_0_43: bool = fn_state.tagchecked;
        // D s_0_44: call CreateAccDescAtomicOp(s_0_40, s_0_41, s_0_42, s_0_43)
        let s_0_44: ProductType9878976b5bcce9c9 = CreateAccDescAtomicOp(
            state,
            tracer,
            s_0_40,
            s_0_41,
            s_0_42,
            s_0_43,
        );
        // D s_0_45: write-var accdesc <= s_0_44
        fn_state.accdesc = s_0_44;
        // D s_0_46: read-var accdesc.1:struct
        let s_0_46: u32 = fn_state.accdesc._1;
        // D s_0_47: call BigEndian(s_0_46)
        let s_0_47: bool = BigEndian(state, tracer, s_0_46);
        // N s_0_48: branch s_0_47 b22 b1
        if s_0_47 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var s2:bv
        let s_1_0: Bits = fn_state.s2;
        // D s_1_1: read-var s1:bv
        let s_1_1: Bits = fn_state.s1;
        // D s_1_2: cast reint s_1_0 -> u128
        let s_1_2: u128 = (s_1_0.value() as u128);
        // D s_1_3: size-of s_1_0
        let s_1_3: u16 = s_1_0.length();
        // D s_1_4: cast reint s_1_1 -> u128
        let s_1_4: u128 = (s_1_1.value() as u128);
        // D s_1_5: size-of s_1_1
        let s_1_5: u16 = s_1_1.length();
        // D s_1_6: lsl s_1_2 s_1_5
        let s_1_6: u128 = s_1_2 << s_1_5;
        // D s_1_7: or s_1_6 s_1_4
        let s_1_7: u128 = ((s_1_6) | (s_1_4));
        // D s_1_8: add s_1_3 s_1_5
        let s_1_8: u16 = (s_1_3 + s_1_5);
        // D s_1_9: create-bits s_1_7 s_1_8
        let s_1_9: Bits = Bits::new(s_1_7, s_1_8);
        // D s_1_10: write-var comparevalue <= s_1_9
        fn_state.comparevalue = s_1_9;
        // N s_1_11: jump b2
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
        // N s_2_2: branch s_2_1 b21 b3
        if s_2_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var t2:bv
        let s_3_0: Bits = fn_state.t2;
        // D s_3_1: read-var t1:bv
        let s_3_1: Bits = fn_state.t1;
        // D s_3_2: cast reint s_3_0 -> u128
        let s_3_2: u128 = (s_3_0.value() as u128);
        // D s_3_3: size-of s_3_0
        let s_3_3: u16 = s_3_0.length();
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: lsl s_3_2 s_3_5
        let s_3_6: u128 = s_3_2 << s_3_5;
        // D s_3_7: or s_3_6 s_3_4
        let s_3_7: u128 = ((s_3_6) | (s_3_4));
        // D s_3_8: add s_3_3 s_3_5
        let s_3_8: u16 = (s_3_3 + s_3_5);
        // D s_3_9: create-bits s_3_7 s_3_8
        let s_3_9: Bits = Bits::new(s_3_7, s_3_8);
        // D s_3_10: write-var newvalue <= s_3_9
        fn_state.newvalue = s_3_9;
        // N s_3_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #31s : i
        let s_4_0: i128 = 31;
        // D s_4_1: read-var n:i64
        let s_4_1: i64 = fn_state.n;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // N s_4_4: branch s_4_3 b19 b5
        if s_4_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call X_read(s_5_2, s_5_0)
        let s_5_3: Bits = X_read(state, tracer, s_5_2, s_5_0);
        // D s_5_4: cast reint s_5_3 -> u64
        let s_5_4: u64 = (s_5_3.value() as u64);
        // D s_5_5: write-var address <= s_5_4
        fn_state.address = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var address:u64
        let s_6_0: u64 = fn_state.address;
        // D s_6_1: read-var comparevalue:bv
        let s_6_1: Bits = fn_state.comparevalue;
        // D s_6_2: read-var newvalue:bv
        let s_6_2: Bits = fn_state.newvalue;
        // D s_6_3: read-var accdesc:struct
        let s_6_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_4: call MemAtomic(s_6_0, s_6_1, s_6_2, s_6_3)
        let s_6_4: Bits = MemAtomic(state, tracer, s_6_0, s_6_1, s_6_2, s_6_3);
        // D s_6_5: write-var data <= s_6_4
        fn_state.data = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var accdesc.1:struct
        let s_7_0: u32 = fn_state.accdesc._1;
        // D s_7_1: call BigEndian(s_7_0)
        let s_7_1: bool = BigEndian(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b18 b8
        if s_7_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1112:i64
        let s_8_0: i64 = fn_state.datasizeshadow_1112;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // C s_8_3: const #1s : i
        let s_8_3: i128 = 1;
        // D s_8_4: read-var datasizeshadow#1112:i64
        let s_8_4: i64 = fn_state.datasizeshadow_1112;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: sub s_8_5 s_8_3
        let s_8_6: i128 = ((s_8_5) - (s_8_3));
        // D s_8_7: cast reint s_8_6 -> i64
        let s_8_7: i64 = (s_8_6 as i64);
        // C s_8_8: const #0s : i
        let s_8_8: i128 = 0;
        // D s_8_9: cast zx s_8_7 -> i
        let s_8_9: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_10: read-var data:bv
        let s_8_10: Bits = fn_state.data;
        // C s_8_11: const #1s : i64
        let s_8_11: i64 = 1;
        // C s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: sub s_8_9 s_8_8
        let s_8_13: i128 = ((s_8_9) - (s_8_8));
        // D s_8_14: add s_8_13 s_8_12
        let s_8_14: i128 = (s_8_13 + s_8_12);
        // D s_8_15: bit-extract s_8_10 s_8_8 s_8_14
        let s_8_15: Bits = (Bits::new(
            ((s_8_10) >> (s_8_8)).value(),
            u16::try_from(s_8_14).unwrap(),
        ));
        // D s_8_16: read-var s:i64
        let s_8_16: i64 = fn_state.s;
        // D s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_18: call X_set(s_8_17, s_8_2, s_8_15)
        let s_8_18: () = X_set(state, tracer, s_8_17, s_8_2, s_8_15);
        // C s_8_19: const #1s : i
        let s_8_19: i128 = 1;
        // D s_8_20: read-var s:i64
        let s_8_20: i64 = fn_state.s;
        // D s_8_21: cast zx s_8_20 -> i
        let s_8_21: i128 = (i128::try_from(s_8_20).unwrap());
        // D s_8_22: add s_8_21 s_8_19
        let s_8_22: i128 = (s_8_21 + s_8_19);
        // D s_8_23: cast reint s_8_22 -> i64
        let s_8_23: i64 = (s_8_22 as i64);
        // D s_8_24: read-var datasizeshadow#1112:i64
        let s_8_24: i64 = fn_state.datasizeshadow_1112;
        // D s_8_25: cast zx s_8_24 -> i
        let s_8_25: i128 = (i128::try_from(s_8_24).unwrap());
        // D s_8_26: cast reint s_8_25 -> i64
        let s_8_26: i64 = (s_8_25 as i64);
        // C s_8_27: const #2s : i
        let s_8_27: i128 = 2;
        // D s_8_28: read-var datasizeshadow#1112:i64
        let s_8_28: i64 = fn_state.datasizeshadow_1112;
        // D s_8_29: cast zx s_8_28 -> i
        let s_8_29: i128 = (i128::try_from(s_8_28).unwrap());
        // D s_8_30: mul s_8_27 s_8_29
        let s_8_30: i128 = ((s_8_27) * (s_8_29));
        // D s_8_31: cast reint s_8_30 -> i64
        let s_8_31: i64 = (s_8_30 as i64);
        // C s_8_32: const #1s : i
        let s_8_32: i128 = 1;
        // D s_8_33: cast zx s_8_31 -> i
        let s_8_33: i128 = (i128::try_from(s_8_31).unwrap());
        // D s_8_34: sub s_8_33 s_8_32
        let s_8_34: i128 = ((s_8_33) - (s_8_32));
        // D s_8_35: cast reint s_8_34 -> i64
        let s_8_35: i64 = (s_8_34 as i64);
        // D s_8_36: cast zx s_8_35 -> i
        let s_8_36: i128 = (i128::try_from(s_8_35).unwrap());
        // D s_8_37: read-var datasizeshadow#1112:i64
        let s_8_37: i64 = fn_state.datasizeshadow_1112;
        // D s_8_38: cast zx s_8_37 -> i
        let s_8_38: i128 = (i128::try_from(s_8_37).unwrap());
        // D s_8_39: read-var data:bv
        let s_8_39: Bits = fn_state.data;
        // C s_8_40: const #1s : i64
        let s_8_40: i64 = 1;
        // C s_8_41: cast zx s_8_40 -> i
        let s_8_41: i128 = (i128::try_from(s_8_40).unwrap());
        // D s_8_42: sub s_8_36 s_8_38
        let s_8_42: i128 = ((s_8_36) - (s_8_38));
        // D s_8_43: add s_8_42 s_8_41
        let s_8_43: i128 = (s_8_42 + s_8_41);
        // D s_8_44: bit-extract s_8_39 s_8_38 s_8_43
        let s_8_44: Bits = (Bits::new(
            ((s_8_39) >> (s_8_38)).value(),
            u16::try_from(s_8_43).unwrap(),
        ));
        // D s_8_45: cast zx s_8_23 -> i
        let s_8_45: i128 = (i128::try_from(s_8_23).unwrap());
        // D s_8_46: call X_set(s_8_45, s_8_26, s_8_44)
        let s_8_46: () = X_set(state, tracer, s_8_45, s_8_26, s_8_44);
        // N s_8_47: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #22416u : u32
        let s_9_0: u32 = 22416;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: bool = {
            let value = state.read_register::<bool>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // N s_9_2: branch s_9_1 b11 b10
        if s_9_1 {
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
        // N s_10_0: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var acquire:u8
        let s_11_0: bool = fn_state.acquire;
        // N s_11_1: branch s_11_0 b17 b12
        if s_11_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var release:u8
        let s_12_0: bool = fn_state.release;
        // D s_12_1: write-var gs#145808 <= s_12_0
        fn_state.gs_145808 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#145808:u8
        let s_13_0: bool = fn_state.gs_145808;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var ar <= s_14_0
        fn_state.ar = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ar:u8
        let s_15_0: bool = fn_state.ar;
        // C s_15_1: const #0u : u8
        let s_15_1: bool = false;
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: const #1u : u8
        let s_15_3: bool = true;
        // D s_15_4: call SPESampleExtendedLoadStore(s_15_0, s_15_1, s_15_2, s_15_3)
        let s_15_4: () = SPESampleExtendedLoadStore(
            state,
            tracer,
            s_15_0,
            s_15_1,
            s_15_2,
            s_15_3,
        );
        // N s_15_5: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var ar <= s_16_0
        fn_state.ar = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#145808 <= s_17_0
        fn_state.gs_145808 = s_17_0;
        // N s_17_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var datasizeshadow#1112:i64
        let s_18_0: i64 = fn_state.datasizeshadow_1112;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: cast reint s_18_1 -> i64
        let s_18_2: i64 = (s_18_1 as i64);
        // C s_18_3: const #2s : i
        let s_18_3: i128 = 2;
        // D s_18_4: read-var datasizeshadow#1112:i64
        let s_18_4: i64 = fn_state.datasizeshadow_1112;
        // D s_18_5: cast zx s_18_4 -> i
        let s_18_5: i128 = (i128::try_from(s_18_4).unwrap());
        // D s_18_6: mul s_18_3 s_18_5
        let s_18_6: i128 = ((s_18_3) * (s_18_5));
        // D s_18_7: cast reint s_18_6 -> i64
        let s_18_7: i64 = (s_18_6 as i64);
        // C s_18_8: const #1s : i
        let s_18_8: i128 = 1;
        // D s_18_9: cast zx s_18_7 -> i
        let s_18_9: i128 = (i128::try_from(s_18_7).unwrap());
        // D s_18_10: sub s_18_9 s_18_8
        let s_18_10: i128 = ((s_18_9) - (s_18_8));
        // D s_18_11: cast reint s_18_10 -> i64
        let s_18_11: i64 = (s_18_10 as i64);
        // D s_18_12: cast zx s_18_11 -> i
        let s_18_12: i128 = (i128::try_from(s_18_11).unwrap());
        // D s_18_13: read-var datasizeshadow#1112:i64
        let s_18_13: i64 = fn_state.datasizeshadow_1112;
        // D s_18_14: cast zx s_18_13 -> i
        let s_18_14: i128 = (i128::try_from(s_18_13).unwrap());
        // D s_18_15: read-var data:bv
        let s_18_15: Bits = fn_state.data;
        // C s_18_16: const #1s : i64
        let s_18_16: i64 = 1;
        // C s_18_17: cast zx s_18_16 -> i
        let s_18_17: i128 = (i128::try_from(s_18_16).unwrap());
        // D s_18_18: sub s_18_12 s_18_14
        let s_18_18: i128 = ((s_18_12) - (s_18_14));
        // D s_18_19: add s_18_18 s_18_17
        let s_18_19: i128 = (s_18_18 + s_18_17);
        // D s_18_20: bit-extract s_18_15 s_18_14 s_18_19
        let s_18_20: Bits = (Bits::new(
            ((s_18_15) >> (s_18_14)).value(),
            u16::try_from(s_18_19).unwrap(),
        ));
        // D s_18_21: read-var s:i64
        let s_18_21: i64 = fn_state.s;
        // D s_18_22: cast zx s_18_21 -> i
        let s_18_22: i128 = (i128::try_from(s_18_21).unwrap());
        // D s_18_23: call X_set(s_18_22, s_18_2, s_18_20)
        let s_18_23: () = X_set(state, tracer, s_18_22, s_18_2, s_18_20);
        // C s_18_24: const #1s : i
        let s_18_24: i128 = 1;
        // D s_18_25: read-var s:i64
        let s_18_25: i64 = fn_state.s;
        // D s_18_26: cast zx s_18_25 -> i
        let s_18_26: i128 = (i128::try_from(s_18_25).unwrap());
        // D s_18_27: add s_18_26 s_18_24
        let s_18_27: i128 = (s_18_26 + s_18_24);
        // D s_18_28: cast reint s_18_27 -> i64
        let s_18_28: i64 = (s_18_27 as i64);
        // D s_18_29: read-var datasizeshadow#1112:i64
        let s_18_29: i64 = fn_state.datasizeshadow_1112;
        // D s_18_30: cast zx s_18_29 -> i
        let s_18_30: i128 = (i128::try_from(s_18_29).unwrap());
        // D s_18_31: cast reint s_18_30 -> i64
        let s_18_31: i64 = (s_18_30 as i64);
        // C s_18_32: const #1s : i
        let s_18_32: i128 = 1;
        // D s_18_33: read-var datasizeshadow#1112:i64
        let s_18_33: i64 = fn_state.datasizeshadow_1112;
        // D s_18_34: cast zx s_18_33 -> i
        let s_18_34: i128 = (i128::try_from(s_18_33).unwrap());
        // D s_18_35: sub s_18_34 s_18_32
        let s_18_35: i128 = ((s_18_34) - (s_18_32));
        // D s_18_36: cast reint s_18_35 -> i64
        let s_18_36: i64 = (s_18_35 as i64);
        // C s_18_37: const #0s : i
        let s_18_37: i128 = 0;
        // D s_18_38: cast zx s_18_36 -> i
        let s_18_38: i128 = (i128::try_from(s_18_36).unwrap());
        // D s_18_39: read-var data:bv
        let s_18_39: Bits = fn_state.data;
        // C s_18_40: const #1s : i64
        let s_18_40: i64 = 1;
        // C s_18_41: cast zx s_18_40 -> i
        let s_18_41: i128 = (i128::try_from(s_18_40).unwrap());
        // D s_18_42: sub s_18_38 s_18_37
        let s_18_42: i128 = ((s_18_38) - (s_18_37));
        // D s_18_43: add s_18_42 s_18_41
        let s_18_43: i128 = (s_18_42 + s_18_41);
        // D s_18_44: bit-extract s_18_39 s_18_37 s_18_43
        let s_18_44: Bits = (Bits::new(
            ((s_18_39) >> (s_18_37)).value(),
            u16::try_from(s_18_43).unwrap(),
        ));
        // D s_18_45: cast zx s_18_28 -> i
        let s_18_45: i128 = (i128::try_from(s_18_28).unwrap());
        // D s_18_46: call X_set(s_18_45, s_18_31, s_18_44)
        let s_18_46: () = X_set(state, tracer, s_18_45, s_18_31, s_18_44);
        // N s_18_47: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call CheckSPAlignment(s_19_0)
        let s_19_1: () = CheckSPAlignment(state, tracer, s_19_0);
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call SP_read(s_20_0)
        let s_20_1: u64 = SP_read(state, tracer, s_20_0);
        // D s_20_2: write-var address <= s_20_1
        fn_state.address = s_20_1;
        // N s_20_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var t1:bv
        let s_21_0: Bits = fn_state.t1;
        // D s_21_1: read-var t2:bv
        let s_21_1: Bits = fn_state.t2;
        // D s_21_2: cast reint s_21_0 -> u128
        let s_21_2: u128 = (s_21_0.value() as u128);
        // D s_21_3: size-of s_21_0
        let s_21_3: u16 = s_21_0.length();
        // D s_21_4: cast reint s_21_1 -> u128
        let s_21_4: u128 = (s_21_1.value() as u128);
        // D s_21_5: size-of s_21_1
        let s_21_5: u16 = s_21_1.length();
        // D s_21_6: lsl s_21_2 s_21_5
        let s_21_6: u128 = s_21_2 << s_21_5;
        // D s_21_7: or s_21_6 s_21_4
        let s_21_7: u128 = ((s_21_6) | (s_21_4));
        // D s_21_8: add s_21_3 s_21_5
        let s_21_8: u16 = (s_21_3 + s_21_5);
        // D s_21_9: create-bits s_21_7 s_21_8
        let s_21_9: Bits = Bits::new(s_21_7, s_21_8);
        // D s_21_10: write-var newvalue <= s_21_9
        fn_state.newvalue = s_21_9;
        // N s_21_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var s1:bv
        let s_22_0: Bits = fn_state.s1;
        // D s_22_1: read-var s2:bv
        let s_22_1: Bits = fn_state.s2;
        // D s_22_2: cast reint s_22_0 -> u128
        let s_22_2: u128 = (s_22_0.value() as u128);
        // D s_22_3: size-of s_22_0
        let s_22_3: u16 = s_22_0.length();
        // D s_22_4: cast reint s_22_1 -> u128
        let s_22_4: u128 = (s_22_1.value() as u128);
        // D s_22_5: size-of s_22_1
        let s_22_5: u16 = s_22_1.length();
        // D s_22_6: lsl s_22_2 s_22_5
        let s_22_6: u128 = s_22_2 << s_22_5;
        // D s_22_7: or s_22_6 s_22_4
        let s_22_7: u128 = ((s_22_6) | (s_22_4));
        // D s_22_8: add s_22_3 s_22_5
        let s_22_8: u16 = (s_22_3 + s_22_5);
        // D s_22_9: create-bits s_22_7 s_22_8
        let s_22_9: Bits = Bits::new(s_22_7, s_22_8);
        // D s_22_10: write-var comparevalue <= s_22_9
        fn_state.comparevalue = s_22_9;
        // N s_22_11: jump b2
        return block_2(state, tracer, fn_state);
    }
}
