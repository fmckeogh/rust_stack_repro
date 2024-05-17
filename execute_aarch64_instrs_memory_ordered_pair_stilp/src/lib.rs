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
use Mem_set__2::*;
use HaveLSE2Ext::*;
use SP_set::*;
use SPESampleGeneralPurposeLoadStore::*;
use CreateAccDescAcqRel::*;
use BigEndian::*;
use Mem_read::*;
use X_read::*;
use Mem_read__1::*;
use CheckSPAlignment::*;
use SP_read::*;
use Mem_set::*;
use common::*;
pub fn execute_aarch64_instrs_memory_ordered_pair_stilp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    memop: u32,
    n: i64,
    offset: i128,
    postindex: bool,
    rt_unknown: bool,
    t: i64,
    t2: i64,
    tagchecked: bool,
    wb_unknown: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        full_data: Bits,
        address: u64,
        gs_173305: bool,
        gs_173312: bool,
        data2: Bits,
        dbytes: i64,
        accdesc: ProductType9878976b5bcce9c9,
        data1: Bits,
        datasizeshadow_1983: i64,
        u_2153: Bits,
        gs_173299: bool,
        gs_173300: bool,
        datasize: i64,
        memop: u32,
        n: i64,
        offset: i128,
        postindex: bool,
        rt_unknown: bool,
        t: i64,
        t2: i64,
        tagchecked: bool,
        wb_unknown: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        datasize,
        memop,
        n,
        offset,
        postindex,
        rt_unknown,
        t,
        t2,
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
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1983 <= s_0_2
        fn_state.datasizeshadow_1983 = s_0_2;
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: read-var datasizeshadow#1983:i64
        let s_0_5: i64 = fn_state.datasizeshadow_1983;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: div s_0_6 s_0_4
        let s_0_7: i128 = ((s_0_6) / (s_0_4));
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var dbytes <= s_0_8
        fn_state.dbytes = s_0_8;
        // D s_0_10: read-var memop:u32
        let s_0_10: u32 = fn_state.memop;
        // D s_0_11: read-var tagchecked:u8
        let s_0_11: bool = fn_state.tagchecked;
        // D s_0_12: call CreateAccDescAcqRel(s_0_10, s_0_11)
        let s_0_12: ProductType9878976b5bcce9c9 = CreateAccDescAcqRel(
            state,
            tracer,
            s_0_10,
            s_0_11,
        );
        // D s_0_13: write-var accdesc <= s_0_12
        fn_state.accdesc = s_0_12;
        // C s_0_14: const #31s : i
        let s_0_14: i128 = 31;
        // D s_0_15: read-var n:i64
        let s_0_15: i64 = fn_state.n;
        // D s_0_16: cast zx s_0_15 -> i
        let s_0_16: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_17: cmp-eq s_0_16 s_0_14
        let s_0_17: bool = ((s_0_16) == (s_0_14));
        // N s_0_18: branch s_0_17 b61 b1
        if s_0_17 {
            return block_61(state, tracer, fn_state);
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
        // D s_2_0: read-var postindex:u8
        let s_2_0: bool = fn_state.postindex;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b60 b3
        if s_2_1 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // D s_4_1: read-var memop:u32
        let s_4_1: u32 = fn_state.memop;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b46 b5
        if s_4_3 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var rt_unknown:u8
        let s_5_0: bool = fn_state.rt_unknown;
        // N s_5_1: branch s_5_0 b45 b6
        if s_5_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#173299 <= s_6_0
        fn_state.gs_173299 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#173299:u8
        let s_7_0: bool = fn_state.gs_173299;
        // N s_7_1: branch s_7_0 b44 b8
        if s_7_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1983:i64
        let s_8_0: i64 = fn_state.datasizeshadow_1983;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var t:i64
        let s_8_3: i64 = fn_state.t;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: call X_read(s_8_4, s_8_2)
        let s_8_5: Bits = X_read(state, tracer, s_8_4, s_8_2);
        // D s_8_6: write-var data1 <= s_8_5
        fn_state.data1 = s_8_5;
        // N s_8_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var rt_unknown:u8
        let s_9_0: bool = fn_state.rt_unknown;
        // N s_9_1: branch s_9_0 b43 b10
        if s_9_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#173300 <= s_10_0
        fn_state.gs_173300 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#173300:u8
        let s_11_0: bool = fn_state.gs_173300;
        // N s_11_1: branch s_11_0 b42 b12
        if s_11_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var datasizeshadow#1983:i64
        let s_12_0: i64 = fn_state.datasizeshadow_1983;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var t2:i64
        let s_12_3: i64 = fn_state.t2;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: call X_read(s_12_4, s_12_2)
        let s_12_5: Bits = X_read(state, tracer, s_12_4, s_12_2);
        // D s_12_6: write-var data2 <= s_12_5
        fn_state.data2 = s_12_5;
        // N s_12_7: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HaveLSE2Ext(s_13_0)
        let s_13_1: bool = HaveLSE2Ext(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b35 b14
        if s_13_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var postindex:u8
        let s_14_0: bool = fn_state.postindex;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b34 b15
        if s_14_1 {
            return block_34(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#173305 <= s_15_0
        fn_state.gs_173305 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#173305:u8
        let s_16_0: bool = fn_state.gs_173305;
        // N s_16_1: branch s_16_0 b32 b17
        if s_16_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0s : i
        let s_17_0: i128 = 0;
        // D s_17_1: read-var address:u64
        let s_17_1: u64 = fn_state.address;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 64u16);
        // C s_17_3: cast cvt s_17_0 -> bv
        let s_17_3: Bits = Bits::new(s_17_0 as u128, 128);
        // D s_17_4: add s_17_2 s_17_3
        let s_17_4: Bits = (s_17_2 + s_17_3);
        // D s_17_5: cast reint s_17_4 -> u64
        let s_17_5: u64 = (s_17_4.value() as u64);
        // D s_17_6: read-var dbytes:i64
        let s_17_6: i64 = fn_state.dbytes;
        // D s_17_7: cast zx s_17_6 -> i
        let s_17_7: i128 = (i128::try_from(s_17_6).unwrap());
        // D s_17_8: read-var accdesc:struct
        let s_17_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_17_9: read-var data1:bv
        let s_17_9: Bits = fn_state.data1;
        // D s_17_10: call Mem_set(s_17_5, s_17_7, s_17_8, s_17_9)
        let s_17_10: () = Mem_set(state, tracer, s_17_5, s_17_7, s_17_8, s_17_9);
        // N s_17_11: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var address:u64
        let s_18_0: u64 = fn_state.address;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 64u16);
        // D s_18_2: read-var dbytes:i64
        let s_18_2: i64 = fn_state.dbytes;
        // D s_18_3: cast zx s_18_2 -> i
        let s_18_3: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_4: cast cvt s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 128);
        // D s_18_5: add s_18_1 s_18_4
        let s_18_5: Bits = (s_18_1 + s_18_4);
        // D s_18_6: cast reint s_18_5 -> u64
        let s_18_6: u64 = (s_18_5.value() as u64);
        // D s_18_7: read-var dbytes:i64
        let s_18_7: i64 = fn_state.dbytes;
        // D s_18_8: cast zx s_18_7 -> i
        let s_18_8: i128 = (i128::try_from(s_18_7).unwrap());
        // D s_18_9: read-var accdesc:struct
        let s_18_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_18_10: read-var data2:bv
        let s_18_10: Bits = fn_state.data2;
        // D s_18_11: call Mem_set(s_18_6, s_18_8, s_18_9, s_18_10)
        let s_18_11: () = Mem_set(state, tracer, s_18_6, s_18_8, s_18_9, s_18_10);
        // N s_18_12: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var wback:u8
        let s_19_0: bool = fn_state.wback;
        // N s_19_1: branch s_19_0 b24 b20
        if s_19_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #22416u : u32
        let s_21_0: u32 = 22416;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: bool = {
            let value = state.read_register::<bool>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // N s_21_2: branch s_21_1 b23 b22
        if s_21_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call SPESampleGeneralPurposeLoadStore(s_23_0)
        let s_23_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_23_0);
        // N s_23_2: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var wb_unknown:u8
        let s_24_0: bool = fn_state.wb_unknown;
        // N s_24_1: branch s_24_0 b31 b25
        if s_24_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var postindex:u8
        let s_25_0: bool = fn_state.postindex;
        // N s_25_1: branch s_25_0 b30 b26
        if s_25_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #31s : i
        let s_27_0: i128 = 31;
        // D s_27_1: read-var n:i64
        let s_27_1: i64 = fn_state.n;
        // D s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (i128::try_from(s_27_1).unwrap());
        // D s_27_3: cmp-eq s_27_2 s_27_0
        let s_27_3: bool = ((s_27_2) == (s_27_0));
        // N s_27_4: branch s_27_3 b29 b28
        if s_27_3 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #64s : i64
        let s_28_0: i64 = 64;
        // D s_28_1: read-var n:i64
        let s_28_1: i64 = fn_state.n;
        // D s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (i128::try_from(s_28_1).unwrap());
        // D s_28_3: read-var address:u64
        let s_28_3: u64 = fn_state.address;
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 64u16);
        // D s_28_5: call X_set(s_28_2, s_28_0, s_28_4)
        let s_28_5: () = X_set(state, tracer, s_28_2, s_28_0, s_28_4);
        // N s_28_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var address:u64
        let s_29_0: u64 = fn_state.address;
        // D s_29_1: call SP_set(s_29_0)
        let s_29_1: () = SP_set(state, tracer, s_29_0);
        // N s_29_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var address:u64
        let s_30_0: u64 = fn_state.address;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 64u16);
        // D s_30_2: read-var offset:i
        let s_30_2: i128 = fn_state.offset;
        // D s_30_3: cast cvt s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 128);
        // D s_30_4: add s_30_1 s_30_3
        let s_30_4: Bits = (s_30_1 + s_30_3);
        // D s_30_5: cast reint s_30_4 -> u64
        let s_30_5: u64 = (s_30_4.value() as u64);
        // D s_30_6: write-var address <= s_30_5
        fn_state.address = s_30_5;
        // N s_30_7: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #64s : i64
        let s_31_0: i64 = 64;
        // C s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // S s_31_2: call __UNKNOWN_bits(s_31_1)
        let s_31_2: Bits = u__UNKNOWN_bits(state, tracer, s_31_1);
        // S s_31_3: cast reint s_31_2 -> u64
        let s_31_3: u64 = (s_31_2.value() as u64);
        // D s_31_4: write-var address <= s_31_3
        fn_state.address = s_31_3;
        // N s_31_5: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var address:u64
        let s_32_0: u64 = fn_state.address;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 64u16);
        // D s_32_2: read-var dbytes:i64
        let s_32_2: i64 = fn_state.dbytes;
        // D s_32_3: cast zx s_32_2 -> i
        let s_32_3: i128 = (i128::try_from(s_32_2).unwrap());
        // D s_32_4: cast cvt s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 128);
        // D s_32_5: add s_32_1 s_32_4
        let s_32_5: Bits = (s_32_1 + s_32_4);
        // D s_32_6: cast reint s_32_5 -> u64
        let s_32_6: u64 = (s_32_5.value() as u64);
        // D s_32_7: read-var dbytes:i64
        let s_32_7: i64 = fn_state.dbytes;
        // D s_32_8: cast zx s_32_7 -> i
        let s_32_8: i128 = (i128::try_from(s_32_7).unwrap());
        // D s_32_9: read-var accdesc:struct
        let s_32_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_32_10: read-var data2:bv
        let s_32_10: Bits = fn_state.data2;
        // D s_32_11: call Mem_set(s_32_6, s_32_8, s_32_9, s_32_10)
        let s_32_11: () = Mem_set(state, tracer, s_32_6, s_32_8, s_32_9, s_32_10);
        // N s_32_12: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0s : i
        let s_33_0: i128 = 0;
        // D s_33_1: read-var address:u64
        let s_33_1: u64 = fn_state.address;
        // D s_33_2: cast zx s_33_1 -> bv
        let s_33_2: Bits = Bits::new(s_33_1 as u128, 64u16);
        // C s_33_3: cast cvt s_33_0 -> bv
        let s_33_3: Bits = Bits::new(s_33_0 as u128, 128);
        // D s_33_4: add s_33_2 s_33_3
        let s_33_4: Bits = (s_33_2 + s_33_3);
        // D s_33_5: cast reint s_33_4 -> u64
        let s_33_5: u64 = (s_33_4.value() as u64);
        // D s_33_6: read-var dbytes:i64
        let s_33_6: i64 = fn_state.dbytes;
        // D s_33_7: cast zx s_33_6 -> i
        let s_33_7: i128 = (i128::try_from(s_33_6).unwrap());
        // D s_33_8: read-var accdesc:struct
        let s_33_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_33_9: read-var data1:bv
        let s_33_9: Bits = fn_state.data1;
        // D s_33_10: call Mem_set(s_33_5, s_33_7, s_33_8, s_33_9)
        let s_33_10: () = Mem_set(state, tracer, s_33_5, s_33_7, s_33_8, s_33_9);
        // N s_33_11: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0s : i
        let s_34_0: i128 = 0;
        // D s_34_1: read-var offset:i
        let s_34_1: i128 = fn_state.offset;
        // D s_34_2: cmp-lt s_34_1 s_34_0
        let s_34_2: bool = ((s_34_1) < (s_34_0));
        // D s_34_3: write-var gs#173305 <= s_34_2
        fn_state.gs_173305 = s_34_2;
        // N s_34_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var accdesc.1:struct
        let s_35_0: u32 = fn_state.accdesc._1;
        // D s_35_1: call BigEndian(s_35_0)
        let s_35_1: bool = BigEndian(state, tracer, s_35_0);
        // N s_35_2: branch s_35_1 b41 b36
        if s_35_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var data2:bv
        let s_36_0: Bits = fn_state.data2;
        // D s_36_1: read-var data1:bv
        let s_36_1: Bits = fn_state.data1;
        // D s_36_2: cast reint s_36_0 -> u128
        let s_36_2: u128 = (s_36_0.value() as u128);
        // D s_36_3: size-of s_36_0
        let s_36_3: u16 = s_36_0.length();
        // D s_36_4: cast reint s_36_1 -> u128
        let s_36_4: u128 = (s_36_1.value() as u128);
        // D s_36_5: size-of s_36_1
        let s_36_5: u16 = s_36_1.length();
        // D s_36_6: lsl s_36_2 s_36_5
        let s_36_6: u128 = s_36_2 << s_36_5;
        // D s_36_7: or s_36_6 s_36_4
        let s_36_7: u128 = ((s_36_6) | (s_36_4));
        // D s_36_8: add s_36_3 s_36_5
        let s_36_8: u16 = (s_36_3 + s_36_5);
        // D s_36_9: create-bits s_36_7 s_36_8
        let s_36_9: Bits = Bits::new(s_36_7, s_36_8);
        // D s_36_10: write-var full_data <= s_36_9
        fn_state.full_data = s_36_9;
        // N s_36_11: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var postindex:u8
        let s_37_0: bool = fn_state.postindex;
        // D s_37_1: not s_37_0
        let s_37_1: bool = !s_37_0;
        // N s_37_2: branch s_37_1 b40 b38
        if s_37_1 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#173312 <= s_38_0
        fn_state.gs_173312 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#173312:u8
        let s_39_0: bool = fn_state.gs_173312;
        // C s_39_1: const #2s : i
        let s_39_1: i128 = 2;
        // D s_39_2: read-var dbytes:i64
        let s_39_2: i64 = fn_state.dbytes;
        // D s_39_3: cast zx s_39_2 -> i
        let s_39_3: i128 = (i128::try_from(s_39_2).unwrap());
        // D s_39_4: mul s_39_1 s_39_3
        let s_39_4: i128 = ((s_39_1) * (s_39_3));
        // D s_39_5: cast reint s_39_4 -> i64
        let s_39_5: i64 = (s_39_4 as i64);
        // D s_39_6: cast zx s_39_5 -> i
        let s_39_6: i128 = (i128::try_from(s_39_5).unwrap());
        // D s_39_7: read-var address:u64
        let s_39_7: u64 = fn_state.address;
        // D s_39_8: read-var accdesc:struct
        let s_39_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // C s_39_9: const #1u : u8
        let s_39_9: bool = true;
        // D s_39_10: read-var full_data:bv
        let s_39_10: Bits = fn_state.full_data;
        // D s_39_11: call Mem_set__2(s_39_7, s_39_6, s_39_8, s_39_9, s_39_0, s_39_10)
        let s_39_11: () = Mem_set__2(
            state,
            tracer,
            s_39_7,
            s_39_6,
            s_39_8,
            s_39_9,
            s_39_0,
            s_39_10,
        );
        // N s_39_12: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0s : i
        let s_40_0: i128 = 0;
        // D s_40_1: read-var offset:i
        let s_40_1: i128 = fn_state.offset;
        // D s_40_2: cmp-lt s_40_1 s_40_0
        let s_40_2: bool = ((s_40_1) < (s_40_0));
        // D s_40_3: write-var gs#173312 <= s_40_2
        fn_state.gs_173312 = s_40_2;
        // N s_40_4: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var data1:bv
        let s_41_0: Bits = fn_state.data1;
        // D s_41_1: read-var data2:bv
        let s_41_1: Bits = fn_state.data2;
        // D s_41_2: cast reint s_41_0 -> u128
        let s_41_2: u128 = (s_41_0.value() as u128);
        // D s_41_3: size-of s_41_0
        let s_41_3: u16 = s_41_0.length();
        // D s_41_4: cast reint s_41_1 -> u128
        let s_41_4: u128 = (s_41_1.value() as u128);
        // D s_41_5: size-of s_41_1
        let s_41_5: u16 = s_41_1.length();
        // D s_41_6: lsl s_41_2 s_41_5
        let s_41_6: u128 = s_41_2 << s_41_5;
        // D s_41_7: or s_41_6 s_41_4
        let s_41_7: u128 = ((s_41_6) | (s_41_4));
        // D s_41_8: add s_41_3 s_41_5
        let s_41_8: u16 = (s_41_3 + s_41_5);
        // D s_41_9: create-bits s_41_7 s_41_8
        let s_41_9: Bits = Bits::new(s_41_7, s_41_8);
        // D s_41_10: write-var full_data <= s_41_9
        fn_state.full_data = s_41_9;
        // N s_41_11: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var datasizeshadow#1983:i64
        let s_42_0: i64 = fn_state.datasizeshadow_1983;
        // D s_42_1: cast zx s_42_0 -> i
        let s_42_1: i128 = (i128::try_from(s_42_0).unwrap());
        // D s_42_2: cast reint s_42_1 -> i64
        let s_42_2: i64 = (s_42_1 as i64);
        // D s_42_3: cast zx s_42_2 -> i
        let s_42_3: i128 = (i128::try_from(s_42_2).unwrap());
        // D s_42_4: call __UNKNOWN_bits(s_42_3)
        let s_42_4: Bits = u__UNKNOWN_bits(state, tracer, s_42_3);
        // D s_42_5: write-var data2 <= s_42_4
        fn_state.data2 = s_42_4;
        // N s_42_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var t2:i64
        let s_43_0: i64 = fn_state.t2;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: read-var n:i64
        let s_43_2: i64 = fn_state.n;
        // D s_43_3: cast zx s_43_2 -> i
        let s_43_3: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#173300 <= s_43_4
        fn_state.gs_173300 = s_43_4;
        // N s_43_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var datasizeshadow#1983:i64
        let s_44_0: i64 = fn_state.datasizeshadow_1983;
        // D s_44_1: cast zx s_44_0 -> i
        let s_44_1: i128 = (i128::try_from(s_44_0).unwrap());
        // D s_44_2: cast reint s_44_1 -> i64
        let s_44_2: i64 = (s_44_1 as i64);
        // D s_44_3: cast zx s_44_2 -> i
        let s_44_3: i128 = (i128::try_from(s_44_2).unwrap());
        // D s_44_4: call __UNKNOWN_bits(s_44_3)
        let s_44_4: Bits = u__UNKNOWN_bits(state, tracer, s_44_3);
        // D s_44_5: write-var data1 <= s_44_4
        fn_state.data1 = s_44_4;
        // N s_44_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var t:i64
        let s_45_0: i64 = fn_state.t;
        // D s_45_1: cast zx s_45_0 -> i
        let s_45_1: i128 = (i128::try_from(s_45_0).unwrap());
        // D s_45_2: read-var n:i64
        let s_45_2: i64 = fn_state.n;
        // D s_45_3: cast zx s_45_2 -> i
        let s_45_3: i128 = (i128::try_from(s_45_2).unwrap());
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#173299 <= s_45_4
        fn_state.gs_173299 = s_45_4;
        // N s_45_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u32
        let s_46_0: u32 = 0;
        // D s_46_1: read-var memop:u32
        let s_46_1: u32 = fn_state.memop;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // N s_46_4: branch s_46_3 b59 b47
        if s_46_3 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call HaveLSE2Ext(s_47_0)
        let s_47_1: bool = HaveLSE2Ext(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b55 b48
        if s_47_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0s : i
        let s_48_0: i128 = 0;
        // D s_48_1: read-var address:u64
        let s_48_1: u64 = fn_state.address;
        // D s_48_2: cast zx s_48_1 -> bv
        let s_48_2: Bits = Bits::new(s_48_1 as u128, 64u16);
        // C s_48_3: cast cvt s_48_0 -> bv
        let s_48_3: Bits = Bits::new(s_48_0 as u128, 128);
        // D s_48_4: add s_48_2 s_48_3
        let s_48_4: Bits = (s_48_2 + s_48_3);
        // D s_48_5: cast reint s_48_4 -> u64
        let s_48_5: u64 = (s_48_4.value() as u64);
        // D s_48_6: read-var dbytes:i64
        let s_48_6: i64 = fn_state.dbytes;
        // D s_48_7: cast zx s_48_6 -> i
        let s_48_7: i128 = (i128::try_from(s_48_6).unwrap());
        // D s_48_8: read-var accdesc:struct
        let s_48_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_48_9: call Mem_read(s_48_5, s_48_7, s_48_8)
        let s_48_9: Bits = Mem_read(state, tracer, s_48_5, s_48_7, s_48_8);
        // D s_48_10: write-var data1 <= s_48_9
        fn_state.data1 = s_48_9;
        // N s_48_11: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var address:u64
        let s_49_0: u64 = fn_state.address;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 64u16);
        // D s_49_2: read-var dbytes:i64
        let s_49_2: i64 = fn_state.dbytes;
        // D s_49_3: cast zx s_49_2 -> i
        let s_49_3: i128 = (i128::try_from(s_49_2).unwrap());
        // D s_49_4: cast cvt s_49_3 -> bv
        let s_49_4: Bits = Bits::new(s_49_3 as u128, 128);
        // D s_49_5: add s_49_1 s_49_4
        let s_49_5: Bits = (s_49_1 + s_49_4);
        // D s_49_6: cast reint s_49_5 -> u64
        let s_49_6: u64 = (s_49_5.value() as u64);
        // D s_49_7: read-var dbytes:i64
        let s_49_7: i64 = fn_state.dbytes;
        // D s_49_8: cast zx s_49_7 -> i
        let s_49_8: i128 = (i128::try_from(s_49_7).unwrap());
        // D s_49_9: read-var accdesc:struct
        let s_49_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_49_10: call Mem_read(s_49_6, s_49_8, s_49_9)
        let s_49_10: Bits = Mem_read(state, tracer, s_49_6, s_49_8, s_49_9);
        // D s_49_11: write-var data2 <= s_49_10
        fn_state.data2 = s_49_10;
        // N s_49_12: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var rt_unknown:u8
        let s_51_0: bool = fn_state.rt_unknown;
        // N s_51_1: branch s_51_0 b54 b52
        if s_51_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var datasizeshadow#1983:i64
        let s_53_0: i64 = fn_state.datasizeshadow_1983;
        // D s_53_1: cast zx s_53_0 -> i
        let s_53_1: i128 = (i128::try_from(s_53_0).unwrap());
        // D s_53_2: cast reint s_53_1 -> i64
        let s_53_2: i64 = (s_53_1 as i64);
        // D s_53_3: read-var t:i64
        let s_53_3: i64 = fn_state.t;
        // D s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // D s_53_5: read-var data1:bv
        let s_53_5: Bits = fn_state.data1;
        // D s_53_6: call X_set(s_53_4, s_53_2, s_53_5)
        let s_53_6: () = X_set(state, tracer, s_53_4, s_53_2, s_53_5);
        // D s_53_7: read-var datasizeshadow#1983:i64
        let s_53_7: i64 = fn_state.datasizeshadow_1983;
        // D s_53_8: cast zx s_53_7 -> i
        let s_53_8: i128 = (i128::try_from(s_53_7).unwrap());
        // D s_53_9: cast reint s_53_8 -> i64
        let s_53_9: i64 = (s_53_8 as i64);
        // D s_53_10: read-var t2:i64
        let s_53_10: i64 = fn_state.t2;
        // D s_53_11: cast zx s_53_10 -> i
        let s_53_11: i128 = (i128::try_from(s_53_10).unwrap());
        // D s_53_12: read-var data2:bv
        let s_53_12: Bits = fn_state.data2;
        // D s_53_13: call X_set(s_53_11, s_53_9, s_53_12)
        let s_53_13: () = X_set(state, tracer, s_53_11, s_53_9, s_53_12);
        // N s_53_14: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var datasizeshadow#1983:i64
        let s_54_0: i64 = fn_state.datasizeshadow_1983;
        // D s_54_1: cast zx s_54_0 -> i
        let s_54_1: i128 = (i128::try_from(s_54_0).unwrap());
        // D s_54_2: cast reint s_54_1 -> i64
        let s_54_2: i64 = (s_54_1 as i64);
        // D s_54_3: cast zx s_54_2 -> i
        let s_54_3: i128 = (i128::try_from(s_54_2).unwrap());
        // D s_54_4: call __UNKNOWN_bits(s_54_3)
        let s_54_4: Bits = u__UNKNOWN_bits(state, tracer, s_54_3);
        // D s_54_5: write-var data1 <= s_54_4
        fn_state.data1 = s_54_4;
        // D s_54_6: read-var datasizeshadow#1983:i64
        let s_54_6: i64 = fn_state.datasizeshadow_1983;
        // D s_54_7: cast zx s_54_6 -> i
        let s_54_7: i128 = (i128::try_from(s_54_6).unwrap());
        // D s_54_8: cast reint s_54_7 -> i64
        let s_54_8: i64 = (s_54_7 as i64);
        // D s_54_9: cast zx s_54_8 -> i
        let s_54_9: i128 = (i128::try_from(s_54_8).unwrap());
        // D s_54_10: call __UNKNOWN_bits(s_54_9)
        let s_54_10: Bits = u__UNKNOWN_bits(state, tracer, s_54_9);
        // D s_54_11: write-var data2 <= s_54_10
        fn_state.data2 = s_54_10;
        // N s_54_12: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #2s : i
        let s_55_0: i128 = 2;
        // D s_55_1: read-var dbytes:i64
        let s_55_1: i64 = fn_state.dbytes;
        // D s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (i128::try_from(s_55_1).unwrap());
        // D s_55_3: mul s_55_0 s_55_2
        let s_55_3: i128 = ((s_55_0) * (s_55_2));
        // D s_55_4: cast reint s_55_3 -> i64
        let s_55_4: i64 = (s_55_3 as i64);
        // D s_55_5: cast zx s_55_4 -> i
        let s_55_5: i128 = (i128::try_from(s_55_4).unwrap());
        // D s_55_6: read-var address:u64
        let s_55_6: u64 = fn_state.address;
        // D s_55_7: read-var accdesc:struct
        let s_55_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // C s_55_8: const #1u : u8
        let s_55_8: bool = true;
        // D s_55_9: call Mem_read__1(s_55_6, s_55_5, s_55_7, s_55_8)
        let s_55_9: Bits = Mem_read__1(state, tracer, s_55_6, s_55_5, s_55_7, s_55_8);
        // D s_55_10: write-var u#2153 <= s_55_9
        fn_state.u_2153 = s_55_9;
        // N s_55_11: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var accdesc.1:struct
        let s_56_0: u32 = fn_state.accdesc._1;
        // D s_56_1: call BigEndian(s_56_0)
        let s_56_1: bool = BigEndian(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b58 b57
        if s_56_1 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1s : i
        let s_57_0: i128 = 1;
        // D s_57_1: read-var datasizeshadow#1983:i64
        let s_57_1: i64 = fn_state.datasizeshadow_1983;
        // D s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (i128::try_from(s_57_1).unwrap());
        // D s_57_3: sub s_57_2 s_57_0
        let s_57_3: i128 = ((s_57_2) - (s_57_0));
        // D s_57_4: cast reint s_57_3 -> i64
        let s_57_4: i64 = (s_57_3 as i64);
        // C s_57_5: const #0s : i
        let s_57_5: i128 = 0;
        // D s_57_6: cast zx s_57_4 -> i
        let s_57_6: i128 = (i128::try_from(s_57_4).unwrap());
        // D s_57_7: read-var u#2153:bv
        let s_57_7: Bits = fn_state.u_2153;
        // C s_57_8: const #1s : i64
        let s_57_8: i64 = 1;
        // C s_57_9: cast zx s_57_8 -> i
        let s_57_9: i128 = (i128::try_from(s_57_8).unwrap());
        // D s_57_10: sub s_57_6 s_57_5
        let s_57_10: i128 = ((s_57_6) - (s_57_5));
        // D s_57_11: add s_57_10 s_57_9
        let s_57_11: i128 = (s_57_10 + s_57_9);
        // D s_57_12: bit-extract s_57_7 s_57_5 s_57_11
        let s_57_12: Bits = (Bits::new(
            ((s_57_7) >> (s_57_5)).value(),
            u16::try_from(s_57_11).unwrap(),
        ));
        // D s_57_13: write-var data1 <= s_57_12
        fn_state.data1 = s_57_12;
        // C s_57_14: const #2s : i
        let s_57_14: i128 = 2;
        // D s_57_15: read-var datasizeshadow#1983:i64
        let s_57_15: i64 = fn_state.datasizeshadow_1983;
        // D s_57_16: cast zx s_57_15 -> i
        let s_57_16: i128 = (i128::try_from(s_57_15).unwrap());
        // D s_57_17: mul s_57_14 s_57_16
        let s_57_17: i128 = ((s_57_14) * (s_57_16));
        // D s_57_18: cast reint s_57_17 -> i64
        let s_57_18: i64 = (s_57_17 as i64);
        // C s_57_19: const #1s : i
        let s_57_19: i128 = 1;
        // D s_57_20: cast zx s_57_18 -> i
        let s_57_20: i128 = (i128::try_from(s_57_18).unwrap());
        // D s_57_21: sub s_57_20 s_57_19
        let s_57_21: i128 = ((s_57_20) - (s_57_19));
        // D s_57_22: cast reint s_57_21 -> i64
        let s_57_22: i64 = (s_57_21 as i64);
        // D s_57_23: cast zx s_57_22 -> i
        let s_57_23: i128 = (i128::try_from(s_57_22).unwrap());
        // D s_57_24: read-var datasizeshadow#1983:i64
        let s_57_24: i64 = fn_state.datasizeshadow_1983;
        // D s_57_25: cast zx s_57_24 -> i
        let s_57_25: i128 = (i128::try_from(s_57_24).unwrap());
        // D s_57_26: read-var u#2153:bv
        let s_57_26: Bits = fn_state.u_2153;
        // C s_57_27: const #1s : i64
        let s_57_27: i64 = 1;
        // C s_57_28: cast zx s_57_27 -> i
        let s_57_28: i128 = (i128::try_from(s_57_27).unwrap());
        // D s_57_29: sub s_57_23 s_57_25
        let s_57_29: i128 = ((s_57_23) - (s_57_25));
        // D s_57_30: add s_57_29 s_57_28
        let s_57_30: i128 = (s_57_29 + s_57_28);
        // D s_57_31: bit-extract s_57_26 s_57_25 s_57_30
        let s_57_31: Bits = (Bits::new(
            ((s_57_26) >> (s_57_25)).value(),
            u16::try_from(s_57_30).unwrap(),
        ));
        // D s_57_32: write-var data2 <= s_57_31
        fn_state.data2 = s_57_31;
        // N s_57_33: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1s : i
        let s_58_0: i128 = 1;
        // D s_58_1: read-var datasizeshadow#1983:i64
        let s_58_1: i64 = fn_state.datasizeshadow_1983;
        // D s_58_2: cast zx s_58_1 -> i
        let s_58_2: i128 = (i128::try_from(s_58_1).unwrap());
        // D s_58_3: sub s_58_2 s_58_0
        let s_58_3: i128 = ((s_58_2) - (s_58_0));
        // D s_58_4: cast reint s_58_3 -> i64
        let s_58_4: i64 = (s_58_3 as i64);
        // C s_58_5: const #0s : i
        let s_58_5: i128 = 0;
        // D s_58_6: cast zx s_58_4 -> i
        let s_58_6: i128 = (i128::try_from(s_58_4).unwrap());
        // D s_58_7: read-var u#2153:bv
        let s_58_7: Bits = fn_state.u_2153;
        // C s_58_8: const #1s : i64
        let s_58_8: i64 = 1;
        // C s_58_9: cast zx s_58_8 -> i
        let s_58_9: i128 = (i128::try_from(s_58_8).unwrap());
        // D s_58_10: sub s_58_6 s_58_5
        let s_58_10: i128 = ((s_58_6) - (s_58_5));
        // D s_58_11: add s_58_10 s_58_9
        let s_58_11: i128 = (s_58_10 + s_58_9);
        // D s_58_12: bit-extract s_58_7 s_58_5 s_58_11
        let s_58_12: Bits = (Bits::new(
            ((s_58_7) >> (s_58_5)).value(),
            u16::try_from(s_58_11).unwrap(),
        ));
        // D s_58_13: write-var data2 <= s_58_12
        fn_state.data2 = s_58_12;
        // C s_58_14: const #2s : i
        let s_58_14: i128 = 2;
        // D s_58_15: read-var datasizeshadow#1983:i64
        let s_58_15: i64 = fn_state.datasizeshadow_1983;
        // D s_58_16: cast zx s_58_15 -> i
        let s_58_16: i128 = (i128::try_from(s_58_15).unwrap());
        // D s_58_17: mul s_58_14 s_58_16
        let s_58_17: i128 = ((s_58_14) * (s_58_16));
        // D s_58_18: cast reint s_58_17 -> i64
        let s_58_18: i64 = (s_58_17 as i64);
        // C s_58_19: const #1s : i
        let s_58_19: i128 = 1;
        // D s_58_20: cast zx s_58_18 -> i
        let s_58_20: i128 = (i128::try_from(s_58_18).unwrap());
        // D s_58_21: sub s_58_20 s_58_19
        let s_58_21: i128 = ((s_58_20) - (s_58_19));
        // D s_58_22: cast reint s_58_21 -> i64
        let s_58_22: i64 = (s_58_21 as i64);
        // D s_58_23: cast zx s_58_22 -> i
        let s_58_23: i128 = (i128::try_from(s_58_22).unwrap());
        // D s_58_24: read-var datasizeshadow#1983:i64
        let s_58_24: i64 = fn_state.datasizeshadow_1983;
        // D s_58_25: cast zx s_58_24 -> i
        let s_58_25: i128 = (i128::try_from(s_58_24).unwrap());
        // D s_58_26: read-var u#2153:bv
        let s_58_26: Bits = fn_state.u_2153;
        // C s_58_27: const #1s : i64
        let s_58_27: i64 = 1;
        // C s_58_28: cast zx s_58_27 -> i
        let s_58_28: i128 = (i128::try_from(s_58_27).unwrap());
        // D s_58_29: sub s_58_23 s_58_25
        let s_58_29: i128 = ((s_58_23) - (s_58_25));
        // D s_58_30: add s_58_29 s_58_28
        let s_58_30: i128 = (s_58_29 + s_58_28);
        // D s_58_31: bit-extract s_58_26 s_58_25 s_58_30
        let s_58_31: Bits = (Bits::new(
            ((s_58_26) >> (s_58_25)).value(),
            u16::try_from(s_58_30).unwrap(),
        ));
        // D s_58_32: write-var data1 <= s_58_31
        fn_state.data1 = s_58_31;
        // N s_58_33: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var address:u64
        let s_60_0: u64 = fn_state.address;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 64u16);
        // D s_60_2: read-var offset:i
        let s_60_2: i128 = fn_state.offset;
        // D s_60_3: cast cvt s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 128);
        // D s_60_4: add s_60_1 s_60_3
        let s_60_4: Bits = (s_60_1 + s_60_3);
        // D s_60_5: cast reint s_60_4 -> u64
        let s_60_5: u64 = (s_60_4.value() as u64);
        // D s_60_6: write-var address <= s_60_5
        fn_state.address = s_60_5;
        // N s_60_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call CheckSPAlignment(s_61_0)
        let s_61_1: () = CheckSPAlignment(state, tracer, s_61_0);
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call SP_read(s_62_0)
        let s_62_1: u64 = SP_read(state, tracer, s_62_0);
        // D s_62_2: write-var address <= s_62_1
        fn_state.address = s_62_1;
        // N s_62_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
