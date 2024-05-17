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
use u__id::*;
use SP_read::*;
use X_read::*;
use CheckSPAlignment::*;
use SPESampleExtendedLoadStore::*;
use common::*;
pub fn execute_aarch64_instrs_memory_atomicops_cas_single<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acquire: bool,
    datasize: i64,
    n: i64,
    regsize: i64,
    release: bool,
    s: i64,
    t: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        comparevalue: Bits,
        ar: bool,
        data: Bits,
        gs_145750: bool,
        address: u64,
        newvalue: Bits,
        datasizeshadow_1111: i64,
        regsizeshadow_1110: i64,
        acquire: bool,
        datasize: i64,
        n: i64,
        regsize: i64,
        release: bool,
        s: i64,
        t: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        acquire,
        datasize,
        n,
        regsize,
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
        // D s_0_0: read-var regsize:i64
        let s_0_0: i64 = fn_state.regsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var regsizeshadow#1110 <= s_0_2
        fn_state.regsizeshadow_1110 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1111 <= s_0_6
        fn_state.datasizeshadow_1111 = s_0_6;
        // C s_0_8: const #10u : u32
        let s_0_8: u32 = 10;
        // D s_0_9: read-var acquire:u8
        let s_0_9: bool = fn_state.acquire;
        // D s_0_10: read-var release:u8
        let s_0_10: bool = fn_state.release;
        // D s_0_11: read-var tagchecked:u8
        let s_0_11: bool = fn_state.tagchecked;
        // D s_0_12: call CreateAccDescAtomicOp(s_0_8, s_0_9, s_0_10, s_0_11)
        let s_0_12: ProductType9878976b5bcce9c9 = CreateAccDescAtomicOp(
            state,
            tracer,
            s_0_8,
            s_0_9,
            s_0_10,
            s_0_11,
        );
        // D s_0_13: write-var accdesc <= s_0_12
        fn_state.accdesc = s_0_12;
        // D s_0_14: read-var datasizeshadow#1111:i64
        let s_0_14: i64 = fn_state.datasizeshadow_1111;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: cast reint s_0_15 -> i64
        let s_0_16: i64 = (s_0_15 as i64);
        // D s_0_17: read-var s:i64
        let s_0_17: i64 = fn_state.s;
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: call X_read(s_0_18, s_0_16)
        let s_0_19: Bits = X_read(state, tracer, s_0_18, s_0_16);
        // D s_0_20: write-var comparevalue <= s_0_19
        fn_state.comparevalue = s_0_19;
        // D s_0_21: read-var datasizeshadow#1111:i64
        let s_0_21: i64 = fn_state.datasizeshadow_1111;
        // D s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (i128::try_from(s_0_21).unwrap());
        // D s_0_23: cast reint s_0_22 -> i64
        let s_0_23: i64 = (s_0_22 as i64);
        // D s_0_24: read-var t:i64
        let s_0_24: i64 = fn_state.t;
        // D s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_26: call X_read(s_0_25, s_0_23)
        let s_0_26: Bits = X_read(state, tracer, s_0_25, s_0_23);
        // D s_0_27: write-var newvalue <= s_0_26
        fn_state.newvalue = s_0_26;
        // C s_0_28: const #31s : i
        let s_0_28: i128 = 31;
        // D s_0_29: read-var n:i64
        let s_0_29: i64 = fn_state.n;
        // D s_0_30: cast zx s_0_29 -> i
        let s_0_30: i128 = (i128::try_from(s_0_29).unwrap());
        // D s_0_31: cmp-eq s_0_30 s_0_28
        let s_0_31: bool = ((s_0_30) == (s_0_28));
        // N s_0_32: branch s_0_31 b12 b1
        if s_0_31 {
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
        // D s_2_0: read-var address:u64
        let s_2_0: u64 = fn_state.address;
        // D s_2_1: read-var comparevalue:bv
        let s_2_1: Bits = fn_state.comparevalue;
        // D s_2_2: read-var newvalue:bv
        let s_2_2: Bits = fn_state.newvalue;
        // D s_2_3: read-var accdesc:struct
        let s_2_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_2_4: call MemAtomic(s_2_0, s_2_1, s_2_2, s_2_3)
        let s_2_4: Bits = MemAtomic(state, tracer, s_2_0, s_2_1, s_2_2, s_2_3);
        // D s_2_5: write-var data <= s_2_4
        fn_state.data = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var regsizeshadow#1110:i64
        let s_3_0: i64 = fn_state.regsizeshadow_1110;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call __id(s_3_1)
        let s_3_2: i128 = u__id(state, tracer, s_3_1);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: read-var datasizeshadow#1111:i64
        let s_3_4: i64 = fn_state.datasizeshadow_1111;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: call __id(s_3_5)
        let s_3_6: i128 = u__id(state, tracer, s_3_5);
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: cast zx s_3_3 -> i
        let s_3_8: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_9: cast zx s_3_7 -> i
        let s_3_9: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_10: cmp-ge s_3_8 s_3_9
        let s_3_10: bool = ((s_3_8) >= (s_3_9));
        // N s_3_11: assert s_3_10
        let s_3_11: () = assert!(s_3_10);
        // D s_3_12: read-var regsizeshadow#1110:i64
        let s_3_12: i64 = fn_state.regsizeshadow_1110;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: read-var regsizeshadow#1110:i64
        let s_3_15: i64 = fn_state.regsizeshadow_1110;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: read-var data:bv
        let s_3_17: Bits = fn_state.data;
        // D s_3_18: bits-cast zx s_3_17 -> bv length s_3_16
        let s_3_18: Bits = s_3_17.zero_extend(s_3_16);
        // D s_3_19: read-var s:i64
        let s_3_19: i64 = fn_state.s;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: call X_set(s_3_20, s_3_14, s_3_18)
        let s_3_21: () = X_set(state, tracer, s_3_20, s_3_14, s_3_18);
        // C s_3_22: const #22416u : u32
        let s_3_22: u32 = 22416;
        // D s_3_23: read-reg s_3_22:u8
        let s_3_23: bool = {
            let value = state.read_register::<bool>(s_3_22 as isize);
            tracer.read_register(s_3_22 as isize, value);
            value
        };
        // N s_3_24: branch s_3_23 b5 b4
        if s_3_23 {
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
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var acquire:u8
        let s_5_0: bool = fn_state.acquire;
        // N s_5_1: branch s_5_0 b11 b6
        if s_5_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var release:u8
        let s_6_0: bool = fn_state.release;
        // D s_6_1: write-var gs#145750 <= s_6_0
        fn_state.gs_145750 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#145750:u8
        let s_7_0: bool = fn_state.gs_145750;
        // N s_7_1: branch s_7_0 b10 b8
        if s_7_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var ar <= s_8_0
        fn_state.ar = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ar:u8
        let s_9_0: bool = fn_state.ar;
        // C s_9_1: const #0u : u8
        let s_9_1: bool = false;
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // C s_9_3: const #1u : u8
        let s_9_3: bool = true;
        // D s_9_4: call SPESampleExtendedLoadStore(s_9_0, s_9_1, s_9_2, s_9_3)
        let s_9_4: () = SPESampleExtendedLoadStore(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
        );
        // N s_9_5: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var ar <= s_10_0
        fn_state.ar = s_10_0;
        // N s_10_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#145750 <= s_11_0
        fn_state.gs_145750 = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call CheckSPAlignment(s_12_0)
        let s_12_1: () = CheckSPAlignment(state, tracer, s_12_0);
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call SP_read(s_13_0)
        let s_13_1: u64 = SP_read(state, tracer, s_13_0);
        // D s_13_2: write-var address <= s_13_1
        fn_state.address = s_13_1;
        // N s_13_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
