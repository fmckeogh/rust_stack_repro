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
use SP_read::*;
use HaveLSE2Ext::*;
use SP_set::*;
use SPESampleGeneralPurposeLoadStore::*;
use Mem_set__1::*;
use BigEndian::*;
use Mem_read::*;
use CreateAccDescGPR::*;
use X_read::*;
use Mem_read__1::*;
use CheckSPAlignment::*;
use Mem_set::*;
use common::*;
pub fn execute_aarch64_instrs_memory_pair_general_post_idx<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    memop: u32,
    n: i64,
    nontemporal: bool,
    offset: u64,
    postindex: bool,
    rt_unknown: bool,
    is_signed: bool,
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
        data2: Bits,
        u_2127: Bits,
        dbytes: i64,
        gs_160992: bool,
        accdesc: ProductType9878976b5bcce9c9,
        data1: Bits,
        gs_161001: bool,
        datasizeshadow_1648: i64,
        gs_160993: bool,
        datasize: i64,
        memop: u32,
        n: i64,
        nontemporal: bool,
        offset: u64,
        postindex: bool,
        rt_unknown: bool,
        is_signed: bool,
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
        nontemporal,
        offset,
        postindex,
        rt_unknown,
        is_signed,
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
        // D s_0_3: write-var datasizeshadow#1648 <= s_0_2
        fn_state.datasizeshadow_1648 = s_0_2;
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: read-var datasizeshadow#1648:i64
        let s_0_5: i64 = fn_state.datasizeshadow_1648;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: div s_0_6 s_0_4
        let s_0_7: i128 = ((s_0_6) / (s_0_4));
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var dbytes <= s_0_8
        fn_state.dbytes = s_0_8;
        // C s_0_10: const #16975u : u32
        let s_0_10: u32 = 16975;
        // D s_0_11: read-reg s_0_10:u8
        let s_0_11: u8 = {
            let value = state.read_register::<u8>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-ne s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) != (s_0_15));
        // D s_0_17: read-var memop:u32
        let s_0_17: u32 = fn_state.memop;
        // D s_0_18: read-var nontemporal:u8
        let s_0_18: bool = fn_state.nontemporal;
        // D s_0_19: read-var tagchecked:u8
        let s_0_19: bool = fn_state.tagchecked;
        // D s_0_20: call CreateAccDescGPR(s_0_17, s_0_18, s_0_16, s_0_19)
        let s_0_20: ProductType9878976b5bcce9c9 = CreateAccDescGPR(
            state,
            tracer,
            s_0_17,
            s_0_18,
            s_0_16,
            s_0_19,
        );
        // D s_0_21: write-var accdesc <= s_0_20
        fn_state.accdesc = s_0_20;
        // C s_0_22: const #31s : i
        let s_0_22: i128 = 31;
        // D s_0_23: read-var n:i64
        let s_0_23: i64 = fn_state.n;
        // D s_0_24: cast zx s_0_23 -> i
        let s_0_24: i128 = (i128::try_from(s_0_23).unwrap());
        // D s_0_25: cmp-eq s_0_24 s_0_22
        let s_0_25: bool = ((s_0_24) == (s_0_22));
        // N s_0_26: branch s_0_25 b57 b1
        if s_0_25 {
            return block_57(state, tracer, fn_state);
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
        // N s_2_2: branch s_2_1 b56 b3
        if s_2_1 {
            return block_56(state, tracer, fn_state);
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
        // N s_4_4: branch s_4_3 b37 b5
        if s_4_3 {
            return block_37(state, tracer, fn_state);
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
        // N s_5_1: branch s_5_0 b36 b6
        if s_5_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#160992 <= s_6_0
        fn_state.gs_160992 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#160992:u8
        let s_7_0: bool = fn_state.gs_160992;
        // N s_7_1: branch s_7_0 b35 b8
        if s_7_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1648:i64
        let s_8_0: i64 = fn_state.datasizeshadow_1648;
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
        // N s_9_1: branch s_9_0 b34 b10
        if s_9_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#160993 <= s_10_0
        fn_state.gs_160993 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#160993:u8
        let s_11_0: bool = fn_state.gs_160993;
        // N s_11_1: branch s_11_0 b33 b12
        if s_11_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var datasizeshadow#1648:i64
        let s_12_0: i64 = fn_state.datasizeshadow_1648;
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
        // N s_13_2: branch s_13_1 b29 b14
        if s_13_1 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var address:u64
        let s_14_1: u64 = fn_state.address;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 64u16);
        // C s_14_3: cast cvt s_14_0 -> bv
        let s_14_3: Bits = Bits::new(s_14_0 as u128, 128);
        // D s_14_4: add s_14_2 s_14_3
        let s_14_4: Bits = (s_14_2 + s_14_3);
        // D s_14_5: cast reint s_14_4 -> u64
        let s_14_5: u64 = (s_14_4.value() as u64);
        // D s_14_6: read-var dbytes:i64
        let s_14_6: i64 = fn_state.dbytes;
        // D s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // D s_14_8: read-var accdesc:struct
        let s_14_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_9: read-var data1:bv
        let s_14_9: Bits = fn_state.data1;
        // D s_14_10: call Mem_set(s_14_5, s_14_7, s_14_8, s_14_9)
        let s_14_10: () = Mem_set(state, tracer, s_14_5, s_14_7, s_14_8, s_14_9);
        // N s_14_11: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var address:u64
        let s_15_0: u64 = fn_state.address;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 64u16);
        // D s_15_2: read-var dbytes:i64
        let s_15_2: i64 = fn_state.dbytes;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: cast cvt s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 128);
        // D s_15_5: add s_15_1 s_15_4
        let s_15_5: Bits = (s_15_1 + s_15_4);
        // D s_15_6: cast reint s_15_5 -> u64
        let s_15_6: u64 = (s_15_5.value() as u64);
        // D s_15_7: read-var dbytes:i64
        let s_15_7: i64 = fn_state.dbytes;
        // D s_15_8: cast zx s_15_7 -> i
        let s_15_8: i128 = (i128::try_from(s_15_7).unwrap());
        // D s_15_9: read-var accdesc:struct
        let s_15_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_15_10: read-var data2:bv
        let s_15_10: Bits = fn_state.data2;
        // D s_15_11: call Mem_set(s_15_6, s_15_8, s_15_9, s_15_10)
        let s_15_11: () = Mem_set(state, tracer, s_15_6, s_15_8, s_15_9, s_15_10);
        // N s_15_12: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var wback:u8
        let s_16_0: bool = fn_state.wback;
        // N s_16_1: branch s_16_0 b21 b17
        if s_16_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #22416u : u32
        let s_18_0: u32 = 22416;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: bool = {
            let value = state.read_register::<bool>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // N s_18_2: branch s_18_1 b20 b19
        if s_18_1 {
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
        // N s_19_0: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call SPESampleGeneralPurposeLoadStore(s_20_0)
        let s_20_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_20_0);
        // N s_20_2: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var wb_unknown:u8
        let s_21_0: bool = fn_state.wb_unknown;
        // N s_21_1: branch s_21_0 b28 b22
        if s_21_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var postindex:u8
        let s_22_0: bool = fn_state.postindex;
        // N s_22_1: branch s_22_0 b27 b23
        if s_22_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #31s : i
        let s_24_0: i128 = 31;
        // D s_24_1: read-var n:i64
        let s_24_1: i64 = fn_state.n;
        // D s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // D s_24_3: cmp-eq s_24_2 s_24_0
        let s_24_3: bool = ((s_24_2) == (s_24_0));
        // N s_24_4: branch s_24_3 b26 b25
        if s_24_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // D s_25_1: read-var n:i64
        let s_25_1: i64 = fn_state.n;
        // D s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (i128::try_from(s_25_1).unwrap());
        // D s_25_3: read-var address:u64
        let s_25_3: u64 = fn_state.address;
        // D s_25_4: cast zx s_25_3 -> bv
        let s_25_4: Bits = Bits::new(s_25_3 as u128, 64u16);
        // D s_25_5: call X_set(s_25_2, s_25_0, s_25_4)
        let s_25_5: () = X_set(state, tracer, s_25_2, s_25_0, s_25_4);
        // N s_25_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var address:u64
        let s_26_0: u64 = fn_state.address;
        // D s_26_1: call SP_set(s_26_0)
        let s_26_1: () = SP_set(state, tracer, s_26_0);
        // N s_26_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var address:u64
        let s_27_0: u64 = fn_state.address;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 64u16);
        // D s_27_2: read-var offset:u64
        let s_27_2: u64 = fn_state.offset;
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 64u16);
        // D s_27_4: add s_27_1 s_27_3
        let s_27_4: Bits = (s_27_1 + s_27_3);
        // D s_27_5: cast reint s_27_4 -> u64
        let s_27_5: u64 = (s_27_4.value() as u64);
        // D s_27_6: write-var address <= s_27_5
        fn_state.address = s_27_5;
        // N s_27_7: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #64s : i64
        let s_28_0: i64 = 64;
        // C s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // S s_28_2: call __UNKNOWN_bits(s_28_1)
        let s_28_2: Bits = u__UNKNOWN_bits(state, tracer, s_28_1);
        // S s_28_3: cast reint s_28_2 -> u64
        let s_28_3: u64 = (s_28_2.value() as u64);
        // D s_28_4: write-var address <= s_28_3
        fn_state.address = s_28_3;
        // N s_28_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var accdesc.1:struct
        let s_29_0: u32 = fn_state.accdesc._1;
        // D s_29_1: call BigEndian(s_29_0)
        let s_29_1: bool = BigEndian(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b32 b30
        if s_29_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var data2:bv
        let s_30_0: Bits = fn_state.data2;
        // D s_30_1: read-var data1:bv
        let s_30_1: Bits = fn_state.data1;
        // D s_30_2: cast reint s_30_0 -> u128
        let s_30_2: u128 = (s_30_0.value() as u128);
        // D s_30_3: size-of s_30_0
        let s_30_3: u16 = s_30_0.length();
        // D s_30_4: cast reint s_30_1 -> u128
        let s_30_4: u128 = (s_30_1.value() as u128);
        // D s_30_5: size-of s_30_1
        let s_30_5: u16 = s_30_1.length();
        // D s_30_6: lsl s_30_2 s_30_5
        let s_30_6: u128 = s_30_2 << s_30_5;
        // D s_30_7: or s_30_6 s_30_4
        let s_30_7: u128 = ((s_30_6) | (s_30_4));
        // D s_30_8: add s_30_3 s_30_5
        let s_30_8: u16 = (s_30_3 + s_30_5);
        // D s_30_9: create-bits s_30_7 s_30_8
        let s_30_9: Bits = Bits::new(s_30_7, s_30_8);
        // D s_30_10: write-var full_data <= s_30_9
        fn_state.full_data = s_30_9;
        // N s_30_11: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #2s : i
        let s_31_0: i128 = 2;
        // D s_31_1: read-var dbytes:i64
        let s_31_1: i64 = fn_state.dbytes;
        // D s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (i128::try_from(s_31_1).unwrap());
        // D s_31_3: mul s_31_0 s_31_2
        let s_31_3: i128 = ((s_31_0) * (s_31_2));
        // D s_31_4: cast reint s_31_3 -> i64
        let s_31_4: i64 = (s_31_3 as i64);
        // D s_31_5: cast zx s_31_4 -> i
        let s_31_5: i128 = (i128::try_from(s_31_4).unwrap());
        // D s_31_6: read-var address:u64
        let s_31_6: u64 = fn_state.address;
        // D s_31_7: read-var accdesc:struct
        let s_31_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // C s_31_8: const #1u : u8
        let s_31_8: bool = true;
        // D s_31_9: read-var full_data:bv
        let s_31_9: Bits = fn_state.full_data;
        // D s_31_10: call Mem_set__1(s_31_6, s_31_5, s_31_7, s_31_8, s_31_9)
        let s_31_10: () = Mem_set__1(
            state,
            tracer,
            s_31_6,
            s_31_5,
            s_31_7,
            s_31_8,
            s_31_9,
        );
        // N s_31_11: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var data1:bv
        let s_32_0: Bits = fn_state.data1;
        // D s_32_1: read-var data2:bv
        let s_32_1: Bits = fn_state.data2;
        // D s_32_2: cast reint s_32_0 -> u128
        let s_32_2: u128 = (s_32_0.value() as u128);
        // D s_32_3: size-of s_32_0
        let s_32_3: u16 = s_32_0.length();
        // D s_32_4: cast reint s_32_1 -> u128
        let s_32_4: u128 = (s_32_1.value() as u128);
        // D s_32_5: size-of s_32_1
        let s_32_5: u16 = s_32_1.length();
        // D s_32_6: lsl s_32_2 s_32_5
        let s_32_6: u128 = s_32_2 << s_32_5;
        // D s_32_7: or s_32_6 s_32_4
        let s_32_7: u128 = ((s_32_6) | (s_32_4));
        // D s_32_8: add s_32_3 s_32_5
        let s_32_8: u16 = (s_32_3 + s_32_5);
        // D s_32_9: create-bits s_32_7 s_32_8
        let s_32_9: Bits = Bits::new(s_32_7, s_32_8);
        // D s_32_10: write-var full_data <= s_32_9
        fn_state.full_data = s_32_9;
        // N s_32_11: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var datasizeshadow#1648:i64
        let s_33_0: i64 = fn_state.datasizeshadow_1648;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: cast reint s_33_1 -> i64
        let s_33_2: i64 = (s_33_1 as i64);
        // D s_33_3: cast zx s_33_2 -> i
        let s_33_3: i128 = (i128::try_from(s_33_2).unwrap());
        // D s_33_4: call __UNKNOWN_bits(s_33_3)
        let s_33_4: Bits = u__UNKNOWN_bits(state, tracer, s_33_3);
        // D s_33_5: write-var data2 <= s_33_4
        fn_state.data2 = s_33_4;
        // N s_33_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var t2:i64
        let s_34_0: i64 = fn_state.t2;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: read-var n:i64
        let s_34_2: i64 = fn_state.n;
        // D s_34_3: cast zx s_34_2 -> i
        let s_34_3: i128 = (i128::try_from(s_34_2).unwrap());
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#160993 <= s_34_4
        fn_state.gs_160993 = s_34_4;
        // N s_34_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var datasizeshadow#1648:i64
        let s_35_0: i64 = fn_state.datasizeshadow_1648;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: cast reint s_35_1 -> i64
        let s_35_2: i64 = (s_35_1 as i64);
        // D s_35_3: cast zx s_35_2 -> i
        let s_35_3: i128 = (i128::try_from(s_35_2).unwrap());
        // D s_35_4: call __UNKNOWN_bits(s_35_3)
        let s_35_4: Bits = u__UNKNOWN_bits(state, tracer, s_35_3);
        // D s_35_5: write-var data1 <= s_35_4
        fn_state.data1 = s_35_4;
        // N s_35_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var t:i64
        let s_36_0: i64 = fn_state.t;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: read-var n:i64
        let s_36_2: i64 = fn_state.n;
        // D s_36_3: cast zx s_36_2 -> i
        let s_36_3: i128 = (i128::try_from(s_36_2).unwrap());
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#160992 <= s_36_4
        fn_state.gs_160992 = s_36_4;
        // N s_36_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u32
        let s_37_0: u32 = 0;
        // D s_37_1: read-var memop:u32
        let s_37_1: u32 = fn_state.memop;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // N s_37_4: branch s_37_3 b55 b38
        if s_37_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call HaveLSE2Ext(s_38_0)
        let s_38_1: bool = HaveLSE2Ext(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b54 b39
        if s_38_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#161001 <= s_39_0
        fn_state.gs_161001 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#161001:u8
        let s_40_0: bool = fn_state.gs_161001;
        // N s_40_1: branch s_40_0 b50 b41
        if s_40_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0s : i
        let s_41_0: i128 = 0;
        // D s_41_1: read-var address:u64
        let s_41_1: u64 = fn_state.address;
        // D s_41_2: cast zx s_41_1 -> bv
        let s_41_2: Bits = Bits::new(s_41_1 as u128, 64u16);
        // C s_41_3: cast cvt s_41_0 -> bv
        let s_41_3: Bits = Bits::new(s_41_0 as u128, 128);
        // D s_41_4: add s_41_2 s_41_3
        let s_41_4: Bits = (s_41_2 + s_41_3);
        // D s_41_5: cast reint s_41_4 -> u64
        let s_41_5: u64 = (s_41_4.value() as u64);
        // D s_41_6: read-var dbytes:i64
        let s_41_6: i64 = fn_state.dbytes;
        // D s_41_7: cast zx s_41_6 -> i
        let s_41_7: i128 = (i128::try_from(s_41_6).unwrap());
        // D s_41_8: read-var accdesc:struct
        let s_41_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_41_9: call Mem_read(s_41_5, s_41_7, s_41_8)
        let s_41_9: Bits = Mem_read(state, tracer, s_41_5, s_41_7, s_41_8);
        // D s_41_10: write-var data1 <= s_41_9
        fn_state.data1 = s_41_9;
        // N s_41_11: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var address:u64
        let s_42_0: u64 = fn_state.address;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 64u16);
        // D s_42_2: read-var dbytes:i64
        let s_42_2: i64 = fn_state.dbytes;
        // D s_42_3: cast zx s_42_2 -> i
        let s_42_3: i128 = (i128::try_from(s_42_2).unwrap());
        // D s_42_4: cast cvt s_42_3 -> bv
        let s_42_4: Bits = Bits::new(s_42_3 as u128, 128);
        // D s_42_5: add s_42_1 s_42_4
        let s_42_5: Bits = (s_42_1 + s_42_4);
        // D s_42_6: cast reint s_42_5 -> u64
        let s_42_6: u64 = (s_42_5.value() as u64);
        // D s_42_7: read-var dbytes:i64
        let s_42_7: i64 = fn_state.dbytes;
        // D s_42_8: cast zx s_42_7 -> i
        let s_42_8: i128 = (i128::try_from(s_42_7).unwrap());
        // D s_42_9: read-var accdesc:struct
        let s_42_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_42_10: call Mem_read(s_42_6, s_42_8, s_42_9)
        let s_42_10: Bits = Mem_read(state, tracer, s_42_6, s_42_8, s_42_9);
        // D s_42_11: write-var data2 <= s_42_10
        fn_state.data2 = s_42_10;
        // N s_42_12: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var rt_unknown:u8
        let s_44_0: bool = fn_state.rt_unknown;
        // N s_44_1: branch s_44_0 b49 b45
        if s_44_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var is_signed:u8
        let s_46_0: bool = fn_state.is_signed;
        // N s_46_1: branch s_46_0 b48 b47
        if s_46_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var datasizeshadow#1648:i64
        let s_47_0: i64 = fn_state.datasizeshadow_1648;
        // D s_47_1: cast zx s_47_0 -> i
        let s_47_1: i128 = (i128::try_from(s_47_0).unwrap());
        // D s_47_2: cast reint s_47_1 -> i64
        let s_47_2: i64 = (s_47_1 as i64);
        // D s_47_3: read-var t:i64
        let s_47_3: i64 = fn_state.t;
        // D s_47_4: cast zx s_47_3 -> i
        let s_47_4: i128 = (i128::try_from(s_47_3).unwrap());
        // D s_47_5: read-var data1:bv
        let s_47_5: Bits = fn_state.data1;
        // D s_47_6: call X_set(s_47_4, s_47_2, s_47_5)
        let s_47_6: () = X_set(state, tracer, s_47_4, s_47_2, s_47_5);
        // D s_47_7: read-var datasizeshadow#1648:i64
        let s_47_7: i64 = fn_state.datasizeshadow_1648;
        // D s_47_8: cast zx s_47_7 -> i
        let s_47_8: i128 = (i128::try_from(s_47_7).unwrap());
        // D s_47_9: cast reint s_47_8 -> i64
        let s_47_9: i64 = (s_47_8 as i64);
        // D s_47_10: read-var t2:i64
        let s_47_10: i64 = fn_state.t2;
        // D s_47_11: cast zx s_47_10 -> i
        let s_47_11: i128 = (i128::try_from(s_47_10).unwrap());
        // D s_47_12: read-var data2:bv
        let s_47_12: Bits = fn_state.data2;
        // D s_47_13: call X_set(s_47_11, s_47_9, s_47_12)
        let s_47_13: () = X_set(state, tracer, s_47_11, s_47_9, s_47_12);
        // N s_47_14: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #64s : i64
        let s_48_0: i64 = 64;
        // C s_48_1: const #64s : i
        let s_48_1: i128 = 64;
        // D s_48_2: read-var data1:bv
        let s_48_2: Bits = fn_state.data1;
        // D s_48_3: bits-cast sx s_48_2 -> bv length s_48_1
        let s_48_3: Bits = s_48_2.sign_extend(s_48_1);
        // D s_48_4: cast reint s_48_3 -> u64
        let s_48_4: u64 = (s_48_3.value() as u64);
        // D s_48_5: read-var t:i64
        let s_48_5: i64 = fn_state.t;
        // D s_48_6: cast zx s_48_5 -> i
        let s_48_6: i128 = (i128::try_from(s_48_5).unwrap());
        // D s_48_7: cast zx s_48_4 -> bv
        let s_48_7: Bits = Bits::new(s_48_4 as u128, 64u16);
        // D s_48_8: call X_set(s_48_6, s_48_0, s_48_7)
        let s_48_8: () = X_set(state, tracer, s_48_6, s_48_0, s_48_7);
        // C s_48_9: const #64s : i64
        let s_48_9: i64 = 64;
        // C s_48_10: const #64s : i
        let s_48_10: i128 = 64;
        // D s_48_11: read-var data2:bv
        let s_48_11: Bits = fn_state.data2;
        // D s_48_12: bits-cast sx s_48_11 -> bv length s_48_10
        let s_48_12: Bits = s_48_11.sign_extend(s_48_10);
        // D s_48_13: cast reint s_48_12 -> u64
        let s_48_13: u64 = (s_48_12.value() as u64);
        // D s_48_14: read-var t2:i64
        let s_48_14: i64 = fn_state.t2;
        // D s_48_15: cast zx s_48_14 -> i
        let s_48_15: i128 = (i128::try_from(s_48_14).unwrap());
        // D s_48_16: cast zx s_48_13 -> bv
        let s_48_16: Bits = Bits::new(s_48_13 as u128, 64u16);
        // D s_48_17: call X_set(s_48_15, s_48_9, s_48_16)
        let s_48_17: () = X_set(state, tracer, s_48_15, s_48_9, s_48_16);
        // N s_48_18: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var datasizeshadow#1648:i64
        let s_49_0: i64 = fn_state.datasizeshadow_1648;
        // D s_49_1: cast zx s_49_0 -> i
        let s_49_1: i128 = (i128::try_from(s_49_0).unwrap());
        // D s_49_2: cast reint s_49_1 -> i64
        let s_49_2: i64 = (s_49_1 as i64);
        // D s_49_3: cast zx s_49_2 -> i
        let s_49_3: i128 = (i128::try_from(s_49_2).unwrap());
        // D s_49_4: call __UNKNOWN_bits(s_49_3)
        let s_49_4: Bits = u__UNKNOWN_bits(state, tracer, s_49_3);
        // D s_49_5: write-var data1 <= s_49_4
        fn_state.data1 = s_49_4;
        // D s_49_6: read-var datasizeshadow#1648:i64
        let s_49_6: i64 = fn_state.datasizeshadow_1648;
        // D s_49_7: cast zx s_49_6 -> i
        let s_49_7: i128 = (i128::try_from(s_49_6).unwrap());
        // D s_49_8: cast reint s_49_7 -> i64
        let s_49_8: i64 = (s_49_7 as i64);
        // D s_49_9: cast zx s_49_8 -> i
        let s_49_9: i128 = (i128::try_from(s_49_8).unwrap());
        // D s_49_10: call __UNKNOWN_bits(s_49_9)
        let s_49_10: Bits = u__UNKNOWN_bits(state, tracer, s_49_9);
        // D s_49_11: write-var data2 <= s_49_10
        fn_state.data2 = s_49_10;
        // N s_49_12: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #2s : i
        let s_50_0: i128 = 2;
        // D s_50_1: read-var dbytes:i64
        let s_50_1: i64 = fn_state.dbytes;
        // D s_50_2: cast zx s_50_1 -> i
        let s_50_2: i128 = (i128::try_from(s_50_1).unwrap());
        // D s_50_3: mul s_50_0 s_50_2
        let s_50_3: i128 = ((s_50_0) * (s_50_2));
        // D s_50_4: cast reint s_50_3 -> i64
        let s_50_4: i64 = (s_50_3 as i64);
        // D s_50_5: cast zx s_50_4 -> i
        let s_50_5: i128 = (i128::try_from(s_50_4).unwrap());
        // D s_50_6: read-var address:u64
        let s_50_6: u64 = fn_state.address;
        // D s_50_7: read-var accdesc:struct
        let s_50_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // C s_50_8: const #1u : u8
        let s_50_8: bool = true;
        // D s_50_9: call Mem_read__1(s_50_6, s_50_5, s_50_7, s_50_8)
        let s_50_9: Bits = Mem_read__1(state, tracer, s_50_6, s_50_5, s_50_7, s_50_8);
        // D s_50_10: write-var u#2127 <= s_50_9
        fn_state.u_2127 = s_50_9;
        // N s_50_11: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var accdesc.1:struct
        let s_51_0: u32 = fn_state.accdesc._1;
        // D s_51_1: call BigEndian(s_51_0)
        let s_51_1: bool = BigEndian(state, tracer, s_51_0);
        // N s_51_2: branch s_51_1 b53 b52
        if s_51_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1s : i
        let s_52_0: i128 = 1;
        // D s_52_1: read-var datasizeshadow#1648:i64
        let s_52_1: i64 = fn_state.datasizeshadow_1648;
        // D s_52_2: cast zx s_52_1 -> i
        let s_52_2: i128 = (i128::try_from(s_52_1).unwrap());
        // D s_52_3: sub s_52_2 s_52_0
        let s_52_3: i128 = ((s_52_2) - (s_52_0));
        // D s_52_4: cast reint s_52_3 -> i64
        let s_52_4: i64 = (s_52_3 as i64);
        // C s_52_5: const #0s : i
        let s_52_5: i128 = 0;
        // D s_52_6: cast zx s_52_4 -> i
        let s_52_6: i128 = (i128::try_from(s_52_4).unwrap());
        // D s_52_7: read-var u#2127:bv
        let s_52_7: Bits = fn_state.u_2127;
        // C s_52_8: const #1s : i64
        let s_52_8: i64 = 1;
        // C s_52_9: cast zx s_52_8 -> i
        let s_52_9: i128 = (i128::try_from(s_52_8).unwrap());
        // D s_52_10: sub s_52_6 s_52_5
        let s_52_10: i128 = ((s_52_6) - (s_52_5));
        // D s_52_11: add s_52_10 s_52_9
        let s_52_11: i128 = (s_52_10 + s_52_9);
        // D s_52_12: bit-extract s_52_7 s_52_5 s_52_11
        let s_52_12: Bits = (Bits::new(
            ((s_52_7) >> (s_52_5)).value(),
            u16::try_from(s_52_11).unwrap(),
        ));
        // D s_52_13: write-var data1 <= s_52_12
        fn_state.data1 = s_52_12;
        // C s_52_14: const #2s : i
        let s_52_14: i128 = 2;
        // D s_52_15: read-var datasizeshadow#1648:i64
        let s_52_15: i64 = fn_state.datasizeshadow_1648;
        // D s_52_16: cast zx s_52_15 -> i
        let s_52_16: i128 = (i128::try_from(s_52_15).unwrap());
        // D s_52_17: mul s_52_14 s_52_16
        let s_52_17: i128 = ((s_52_14) * (s_52_16));
        // D s_52_18: cast reint s_52_17 -> i64
        let s_52_18: i64 = (s_52_17 as i64);
        // C s_52_19: const #1s : i
        let s_52_19: i128 = 1;
        // D s_52_20: cast zx s_52_18 -> i
        let s_52_20: i128 = (i128::try_from(s_52_18).unwrap());
        // D s_52_21: sub s_52_20 s_52_19
        let s_52_21: i128 = ((s_52_20) - (s_52_19));
        // D s_52_22: cast reint s_52_21 -> i64
        let s_52_22: i64 = (s_52_21 as i64);
        // D s_52_23: cast zx s_52_22 -> i
        let s_52_23: i128 = (i128::try_from(s_52_22).unwrap());
        // D s_52_24: read-var datasizeshadow#1648:i64
        let s_52_24: i64 = fn_state.datasizeshadow_1648;
        // D s_52_25: cast zx s_52_24 -> i
        let s_52_25: i128 = (i128::try_from(s_52_24).unwrap());
        // D s_52_26: read-var u#2127:bv
        let s_52_26: Bits = fn_state.u_2127;
        // C s_52_27: const #1s : i64
        let s_52_27: i64 = 1;
        // C s_52_28: cast zx s_52_27 -> i
        let s_52_28: i128 = (i128::try_from(s_52_27).unwrap());
        // D s_52_29: sub s_52_23 s_52_25
        let s_52_29: i128 = ((s_52_23) - (s_52_25));
        // D s_52_30: add s_52_29 s_52_28
        let s_52_30: i128 = (s_52_29 + s_52_28);
        // D s_52_31: bit-extract s_52_26 s_52_25 s_52_30
        let s_52_31: Bits = (Bits::new(
            ((s_52_26) >> (s_52_25)).value(),
            u16::try_from(s_52_30).unwrap(),
        ));
        // D s_52_32: write-var data2 <= s_52_31
        fn_state.data2 = s_52_31;
        // N s_52_33: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1s : i
        let s_53_0: i128 = 1;
        // D s_53_1: read-var datasizeshadow#1648:i64
        let s_53_1: i64 = fn_state.datasizeshadow_1648;
        // D s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (i128::try_from(s_53_1).unwrap());
        // D s_53_3: sub s_53_2 s_53_0
        let s_53_3: i128 = ((s_53_2) - (s_53_0));
        // D s_53_4: cast reint s_53_3 -> i64
        let s_53_4: i64 = (s_53_3 as i64);
        // C s_53_5: const #0s : i
        let s_53_5: i128 = 0;
        // D s_53_6: cast zx s_53_4 -> i
        let s_53_6: i128 = (i128::try_from(s_53_4).unwrap());
        // D s_53_7: read-var u#2127:bv
        let s_53_7: Bits = fn_state.u_2127;
        // C s_53_8: const #1s : i64
        let s_53_8: i64 = 1;
        // C s_53_9: cast zx s_53_8 -> i
        let s_53_9: i128 = (i128::try_from(s_53_8).unwrap());
        // D s_53_10: sub s_53_6 s_53_5
        let s_53_10: i128 = ((s_53_6) - (s_53_5));
        // D s_53_11: add s_53_10 s_53_9
        let s_53_11: i128 = (s_53_10 + s_53_9);
        // D s_53_12: bit-extract s_53_7 s_53_5 s_53_11
        let s_53_12: Bits = (Bits::new(
            ((s_53_7) >> (s_53_5)).value(),
            u16::try_from(s_53_11).unwrap(),
        ));
        // D s_53_13: write-var data2 <= s_53_12
        fn_state.data2 = s_53_12;
        // C s_53_14: const #2s : i
        let s_53_14: i128 = 2;
        // D s_53_15: read-var datasizeshadow#1648:i64
        let s_53_15: i64 = fn_state.datasizeshadow_1648;
        // D s_53_16: cast zx s_53_15 -> i
        let s_53_16: i128 = (i128::try_from(s_53_15).unwrap());
        // D s_53_17: mul s_53_14 s_53_16
        let s_53_17: i128 = ((s_53_14) * (s_53_16));
        // D s_53_18: cast reint s_53_17 -> i64
        let s_53_18: i64 = (s_53_17 as i64);
        // C s_53_19: const #1s : i
        let s_53_19: i128 = 1;
        // D s_53_20: cast zx s_53_18 -> i
        let s_53_20: i128 = (i128::try_from(s_53_18).unwrap());
        // D s_53_21: sub s_53_20 s_53_19
        let s_53_21: i128 = ((s_53_20) - (s_53_19));
        // D s_53_22: cast reint s_53_21 -> i64
        let s_53_22: i64 = (s_53_21 as i64);
        // D s_53_23: cast zx s_53_22 -> i
        let s_53_23: i128 = (i128::try_from(s_53_22).unwrap());
        // D s_53_24: read-var datasizeshadow#1648:i64
        let s_53_24: i64 = fn_state.datasizeshadow_1648;
        // D s_53_25: cast zx s_53_24 -> i
        let s_53_25: i128 = (i128::try_from(s_53_24).unwrap());
        // D s_53_26: read-var u#2127:bv
        let s_53_26: Bits = fn_state.u_2127;
        // C s_53_27: const #1s : i64
        let s_53_27: i64 = 1;
        // C s_53_28: cast zx s_53_27 -> i
        let s_53_28: i128 = (i128::try_from(s_53_27).unwrap());
        // D s_53_29: sub s_53_23 s_53_25
        let s_53_29: i128 = ((s_53_23) - (s_53_25));
        // D s_53_30: add s_53_29 s_53_28
        let s_53_30: i128 = (s_53_29 + s_53_28);
        // D s_53_31: bit-extract s_53_26 s_53_25 s_53_30
        let s_53_31: Bits = (Bits::new(
            ((s_53_26) >> (s_53_25)).value(),
            u16::try_from(s_53_30).unwrap(),
        ));
        // D s_53_32: write-var data1 <= s_53_31
        fn_state.data1 = s_53_31;
        // N s_53_33: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var is_signed:u8
        let s_54_0: bool = fn_state.is_signed;
        // D s_54_1: not s_54_0
        let s_54_1: bool = !s_54_0;
        // D s_54_2: write-var gs#161001 <= s_54_1
        fn_state.gs_161001 = s_54_1;
        // N s_54_3: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var address:u64
        let s_56_0: u64 = fn_state.address;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 64u16);
        // D s_56_2: read-var offset:u64
        let s_56_2: u64 = fn_state.offset;
        // D s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 64u16);
        // D s_56_4: add s_56_1 s_56_3
        let s_56_4: Bits = (s_56_1 + s_56_3);
        // D s_56_5: cast reint s_56_4 -> u64
        let s_56_5: u64 = (s_56_4.value() as u64);
        // D s_56_6: write-var address <= s_56_5
        fn_state.address = s_56_5;
        // N s_56_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call CheckSPAlignment(s_57_0)
        let s_57_1: () = CheckSPAlignment(state, tracer, s_57_0);
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call SP_read(s_58_0)
        let s_58_1: u64 = SP_read(state, tracer, s_58_0);
        // D s_58_2: write-var address <= s_58_1
        fn_state.address = s_58_1;
        // N s_58_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
