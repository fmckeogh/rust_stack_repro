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
use X_set::*;
use u__UNKNOWN_bits::*;
use CreateAccDescLDAcqPC::*;
use Mem_read::*;
use u__id::*;
use SP_set::*;
use SP_read::*;
use X_read::*;
use CheckSPAlignment::*;
use SPESampleExtendedLoadStore::*;
use common::*;
pub fn execute_aarch64_instrs_memory_ordered_rcpc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    n: i64,
    offset: i64,
    regsize: i64,
    t: i64,
    tagchecked: bool,
    wb_unknown: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        regsizeshadow_1606: i64,
        data: Bits,
        address: u64,
        dbytes: i64,
        accdesc: ProductType9878976b5bcce9c9,
        datasizeshadow_1607: i64,
        datasize: i64,
        n: i64,
        offset: i64,
        regsize: i64,
        t: i64,
        tagchecked: bool,
        wb_unknown: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        datasize,
        n,
        offset,
        regsize,
        t,
        tagchecked,
        wb_unknown,
        wback,
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
        // D s_0_3: write-var regsizeshadow#1606 <= s_0_2
        fn_state.regsizeshadow_1606 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1607 <= s_0_6
        fn_state.datasizeshadow_1607 = s_0_6;
        // C s_0_8: const #8s : i
        let s_0_8: i128 = 8;
        // D s_0_9: read-var datasizeshadow#1607:i64
        let s_0_9: i64 = fn_state.datasizeshadow_1607;
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_11: div s_0_10 s_0_8
        let s_0_11: i128 = ((s_0_10) / (s_0_8));
        // D s_0_12: cast reint s_0_11 -> i64
        let s_0_12: i64 = (s_0_11 as i64);
        // D s_0_13: write-var dbytes <= s_0_12
        fn_state.dbytes = s_0_12;
        // D s_0_14: read-var tagchecked:u8
        let s_0_14: bool = fn_state.tagchecked;
        // D s_0_15: call CreateAccDescLDAcqPC(s_0_14)
        let s_0_15: ProductType9878976b5bcce9c9 = CreateAccDescLDAcqPC(
            state,
            tracer,
            s_0_14,
        );
        // D s_0_16: write-var accdesc <= s_0_15
        fn_state.accdesc = s_0_15;
        // C s_0_17: const #31s : i
        let s_0_17: i128 = 31;
        // D s_0_18: read-var n:i64
        let s_0_18: i64 = fn_state.n;
        // D s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // D s_0_20: cmp-eq s_0_19 s_0_17
        let s_0_20: bool = ((s_0_19) == (s_0_17));
        // N s_0_21: branch s_0_20 b14 b1
        if s_0_20 {
            return block_14(state, tracer, fn_state);
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
        // D s_2_0: read-var dbytes:i64
        let s_2_0: i64 = fn_state.dbytes;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: read-var address:u64
        let s_2_2: u64 = fn_state.address;
        // D s_2_3: read-var accdesc:struct
        let s_2_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_2_4: call Mem_read(s_2_2, s_2_1, s_2_3)
        let s_2_4: Bits = Mem_read(state, tracer, s_2_2, s_2_1, s_2_3);
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
        // D s_3_0: read-var regsizeshadow#1606:i64
        let s_3_0: i64 = fn_state.regsizeshadow_1606;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call __id(s_3_1)
        let s_3_2: i128 = u__id(state, tracer, s_3_1);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: read-var datasizeshadow#1607:i64
        let s_3_4: i64 = fn_state.datasizeshadow_1607;
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
        // D s_3_12: read-var regsizeshadow#1606:i64
        let s_3_12: i64 = fn_state.regsizeshadow_1606;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: read-var regsizeshadow#1606:i64
        let s_3_15: i64 = fn_state.regsizeshadow_1606;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: read-var data:bv
        let s_3_17: Bits = fn_state.data;
        // D s_3_18: bits-cast zx s_3_17 -> bv length s_3_16
        let s_3_18: Bits = s_3_17.zero_extend(s_3_16);
        // D s_3_19: read-var t:i64
        let s_3_19: i64 = fn_state.t;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: call X_set(s_3_20, s_3_14, s_3_18)
        let s_3_21: () = X_set(state, tracer, s_3_20, s_3_14, s_3_18);
        // D s_3_22: read-var wback:u8
        let s_3_22: bool = fn_state.wback;
        // N s_3_23: branch s_3_22 b8 b4
        if s_3_22 {
            return block_8(state, tracer, fn_state);
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
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // C s_7_1: const #0u : u8
        let s_7_1: bool = false;
        // C s_7_2: const #0u : u8
        let s_7_2: bool = false;
        // C s_7_3: const #1u : u8
        let s_7_3: bool = true;
        // S s_7_4: call SPESampleExtendedLoadStore(s_7_0, s_7_1, s_7_2, s_7_3)
        let s_7_4: () = SPESampleExtendedLoadStore(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
            s_7_3,
        );
        // N s_7_5: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var wb_unknown:u8
        let s_8_0: bool = fn_state.wb_unknown;
        // N s_8_1: branch s_8_0 b13 b9
        if s_8_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var address:u64
        let s_9_0: u64 = fn_state.address;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: read-var offset:i64
        let s_9_2: i64 = fn_state.offset;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cast cvt s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 128);
        // D s_9_5: add s_9_1 s_9_4
        let s_9_5: Bits = (s_9_1 + s_9_4);
        // D s_9_6: cast reint s_9_5 -> u64
        let s_9_6: u64 = (s_9_5.value() as u64);
        // D s_9_7: write-var address <= s_9_6
        fn_state.address = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #31s : i
        let s_10_0: i128 = 31;
        // D s_10_1: read-var n:i64
        let s_10_1: i64 = fn_state.n;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_0
        let s_10_3: bool = ((s_10_2) == (s_10_0));
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
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
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: read-var n:i64
        let s_11_1: i64 = fn_state.n;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: read-var address:u64
        let s_11_3: u64 = fn_state.address;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 64u16);
        // D s_11_5: call X_set(s_11_2, s_11_0, s_11_4)
        let s_11_5: () = X_set(state, tracer, s_11_2, s_11_0, s_11_4);
        // N s_11_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var address:u64
        let s_12_0: u64 = fn_state.address;
        // D s_12_1: call SP_set(s_12_0)
        let s_12_1: () = SP_set(state, tracer, s_12_0);
        // N s_12_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // C s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // S s_13_2: call __UNKNOWN_bits(s_13_1)
        let s_13_2: Bits = u__UNKNOWN_bits(state, tracer, s_13_1);
        // S s_13_3: cast reint s_13_2 -> u64
        let s_13_3: u64 = (s_13_2.value() as u64);
        // D s_13_4: write-var address <= s_13_3
        fn_state.address = s_13_3;
        // N s_13_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call CheckSPAlignment(s_14_0)
        let s_14_1: () = CheckSPAlignment(state, tracer, s_14_0);
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call SP_read(s_15_0)
        let s_15_1: u64 = SP_read(state, tracer, s_15_0);
        // D s_15_2: write-var address <= s_15_1
        fn_state.address = s_15_1;
        // N s_15_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
