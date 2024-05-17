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
use neq_int::*;
use u__id::*;
use X_set::*;
use SP_read::*;
use X_read::*;
use CheckSPAlignment::*;
use SPESampleExtendedLoadStore::*;
use common::*;
pub fn execute_aarch64_instrs_memory_atomicops_ld<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acquire: bool,
    datasize: i64,
    is_load: bool,
    n: i64,
    op: u32,
    regsize: i64,
    release: bool,
    s: i64,
    t: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1575: i64,
        data: Bits,
        gs_157709: bool,
        gs_157706: bool,
        address: u64,
        value_name: Bits,
        regsizeshadow_1574: i64,
        accdesc: ProductType9878976b5bcce9c9,
        ar: bool,
        acquire: bool,
        datasize: i64,
        is_load: bool,
        n: i64,
        op: u32,
        regsize: i64,
        release: bool,
        s: i64,
        t: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        acquire,
        datasize,
        is_load,
        n,
        op,
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
        // D s_0_3: write-var regsizeshadow#1574 <= s_0_2
        fn_state.regsizeshadow_1574 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1575 <= s_0_6
        fn_state.datasizeshadow_1575 = s_0_6;
        // D s_0_8: read-var op:u32
        let s_0_8: u32 = fn_state.op;
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
        // D s_0_14: read-var datasizeshadow#1575:i64
        let s_0_14: i64 = fn_state.datasizeshadow_1575;
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
        // D s_0_20: write-var value_name <= s_0_19
        fn_state.value_name = s_0_19;
        // C s_0_21: const #31s : i
        let s_0_21: i128 = 31;
        // D s_0_22: read-var n:i64
        let s_0_22: i64 = fn_state.n;
        // D s_0_23: cast zx s_0_22 -> i
        let s_0_23: i128 = (i128::try_from(s_0_22).unwrap());
        // D s_0_24: cmp-eq s_0_23 s_0_21
        let s_0_24: bool = ((s_0_23) == (s_0_21));
        // N s_0_25: branch s_0_24 b18 b1
        if s_0_24 {
            return block_18(state, tracer, fn_state);
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
        // D s_2_0: read-var datasizeshadow#1575:i64
        let s_2_0: i64 = fn_state.datasizeshadow_1575;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: call __UNKNOWN_bits(s_2_3)
        let s_2_4: Bits = u__UNKNOWN_bits(state, tracer, s_2_3);
        // D s_2_5: read-var address:u64
        let s_2_5: u64 = fn_state.address;
        // D s_2_6: read-var value_name:bv
        let s_2_6: Bits = fn_state.value_name;
        // D s_2_7: read-var accdesc:struct
        let s_2_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_2_8: call MemAtomic(s_2_5, s_2_4, s_2_6, s_2_7)
        let s_2_8: Bits = MemAtomic(state, tracer, s_2_5, s_2_4, s_2_6, s_2_7);
        // D s_2_9: write-var data <= s_2_8
        fn_state.data = s_2_8;
        // N s_2_10: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #31s : i
        let s_3_0: i128 = 31;
        // D s_3_1: read-var t:i64
        let s_3_1: i64 = fn_state.t;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: call neq_int(s_3_2, s_3_0)
        let s_3_3: bool = neq_int(state, tracer, s_3_2, s_3_0);
        // N s_3_4: branch s_3_3 b14 b4
        if s_3_3 {
            return block_14(state, tracer, fn_state);
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
        // C s_5_0: const #22416u : u32
        let s_5_0: u32 = 22416;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: bool = {
            let value = state.read_register::<bool>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // N s_5_2: branch s_5_1 b7 b6
        if s_5_1 {
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
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var acquire:u8
        let s_7_0: bool = fn_state.acquire;
        // N s_7_1: branch s_7_0 b13 b8
        if s_7_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var release:u8
        let s_8_0: bool = fn_state.release;
        // D s_8_1: write-var gs#157706 <= s_8_0
        fn_state.gs_157706 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#157706:u8
        let s_9_0: bool = fn_state.gs_157706;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var ar <= s_10_0
        fn_state.ar = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ar:u8
        let s_11_0: bool = fn_state.ar;
        // C s_11_1: const #0u : u8
        let s_11_1: bool = false;
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // D s_11_3: read-var is_load:u8
        let s_11_3: bool = fn_state.is_load;
        // D s_11_4: call SPESampleExtendedLoadStore(s_11_0, s_11_1, s_11_2, s_11_3)
        let s_11_4: () = SPESampleExtendedLoadStore(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
            s_11_3,
        );
        // N s_11_5: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var ar <= s_12_0
        fn_state.ar = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#157706 <= s_13_0
        fn_state.gs_157706 = s_13_0;
        // N s_13_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var datasizeshadow#1575:i64
        let s_14_0: i64 = fn_state.datasizeshadow_1575;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #0s : i
        let s_14_4: i128 = 0;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-ge s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) >= (s_14_4));
        // N s_14_7: branch s_14_6 b17 b15
        if s_14_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#157709 <= s_15_0
        fn_state.gs_157709 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#157709:u8
        let s_16_0: bool = fn_state.gs_157709;
        // N s_16_1: assert s_16_0
        let s_16_1: () = assert!(s_16_0);
        // D s_16_2: read-var regsizeshadow#1574:i64
        let s_16_2: i64 = fn_state.regsizeshadow_1574;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: cast reint s_16_3 -> i64
        let s_16_4: i64 = (s_16_3 as i64);
        // D s_16_5: read-var regsizeshadow#1574:i64
        let s_16_5: i64 = fn_state.regsizeshadow_1574;
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_7: read-var data:bv
        let s_16_7: Bits = fn_state.data;
        // D s_16_8: bits-cast zx s_16_7 -> bv length s_16_6
        let s_16_8: Bits = s_16_7.zero_extend(s_16_6);
        // D s_16_9: read-var t:i64
        let s_16_9: i64 = fn_state.t;
        // D s_16_10: cast zx s_16_9 -> i
        let s_16_10: i128 = (i128::try_from(s_16_9).unwrap());
        // D s_16_11: call X_set(s_16_10, s_16_4, s_16_8)
        let s_16_11: () = X_set(state, tracer, s_16_10, s_16_4, s_16_8);
        // N s_16_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var regsizeshadow#1574:i64
        let s_17_0: i64 = fn_state.regsizeshadow_1574;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call __id(s_17_1)
        let s_17_2: i128 = u__id(state, tracer, s_17_1);
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // D s_17_4: read-var datasizeshadow#1575:i64
        let s_17_4: i64 = fn_state.datasizeshadow_1575;
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: call __id(s_17_5)
        let s_17_6: i128 = u__id(state, tracer, s_17_5);
        // D s_17_7: cast reint s_17_6 -> i64
        let s_17_7: i64 = (s_17_6 as i64);
        // D s_17_8: cast zx s_17_3 -> i
        let s_17_8: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_9: cast zx s_17_7 -> i
        let s_17_9: i128 = (i128::try_from(s_17_7).unwrap());
        // D s_17_10: cmp-ge s_17_8 s_17_9
        let s_17_10: bool = ((s_17_8) >= (s_17_9));
        // D s_17_11: write-var gs#157709 <= s_17_10
        fn_state.gs_157709 = s_17_10;
        // N s_17_12: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call CheckSPAlignment(s_18_0)
        let s_18_1: () = CheckSPAlignment(state, tracer, s_18_0);
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call SP_read(s_19_0)
        let s_19_1: u64 = SP_read(state, tracer, s_19_0);
        // D s_19_2: write-var address <= s_19_1
        fn_state.address = s_19_1;
        // N s_19_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
