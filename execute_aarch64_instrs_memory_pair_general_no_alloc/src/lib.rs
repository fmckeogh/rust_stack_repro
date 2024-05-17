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
use SP_set::*;
use HaveLSE2Ext::*;
use SPESampleGeneralPurposeLoadStore::*;
use Mem_read::*;
use BigEndian::*;
use Mem_read__1::*;
use CreateAccDescGPR::*;
use X_read::*;
use CheckSPAlignment::*;
use Mem_set::*;
use common::*;
pub fn execute_aarch64_instrs_memory_pair_general_no_alloc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    memop: u32,
    n: i64,
    nontemporal: bool,
    offset: u64,
    postindex: bool,
    rt_unknown: bool,
    t: i64,
    t2: i64,
    tagchecked: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_160526: bool,
        full_data: Bits,
        address: u64,
        gs_160525: bool,
        data2: Bits,
        dbytes: i64,
        accdesc: ProductType9878976b5bcce9c9,
        data1: Bits,
        datasizeshadow_1641: i64,
        datasize: i64,
        memop: u32,
        n: i64,
        nontemporal: bool,
        offset: u64,
        postindex: bool,
        rt_unknown: bool,
        t: i64,
        t2: i64,
        tagchecked: bool,
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
        t,
        t2,
        tagchecked,
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
        // D s_0_3: write-var datasizeshadow#1641 <= s_0_2
        fn_state.datasizeshadow_1641 = s_0_2;
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: read-var datasizeshadow#1641:i64
        let s_0_5: i64 = fn_state.datasizeshadow_1641;
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
        // N s_0_26: branch s_0_25 b45 b1
        if s_0_25 {
            return block_45(state, tracer, fn_state);
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
        // N s_2_2: branch s_2_1 b44 b3
        if s_2_1 {
            return block_44(state, tracer, fn_state);
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
        // N s_4_4: branch s_4_3 b30 b5
        if s_4_3 {
            return block_30(state, tracer, fn_state);
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
        // N s_5_1: branch s_5_0 b29 b6
        if s_5_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#160525 <= s_6_0
        fn_state.gs_160525 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#160525:u8
        let s_7_0: bool = fn_state.gs_160525;
        // N s_7_1: branch s_7_0 b28 b8
        if s_7_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1641:i64
        let s_8_0: i64 = fn_state.datasizeshadow_1641;
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
        // N s_9_1: branch s_9_0 b27 b10
        if s_9_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#160526 <= s_10_0
        fn_state.gs_160526 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#160526:u8
        let s_11_0: bool = fn_state.gs_160526;
        // N s_11_1: branch s_11_0 b26 b12
        if s_11_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var datasizeshadow#1641:i64
        let s_12_0: i64 = fn_state.datasizeshadow_1641;
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
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var address:u64
        let s_13_1: u64 = fn_state.address;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 64u16);
        // C s_13_3: cast cvt s_13_0 -> bv
        let s_13_3: Bits = Bits::new(s_13_0 as u128, 128);
        // D s_13_4: add s_13_2 s_13_3
        let s_13_4: Bits = (s_13_2 + s_13_3);
        // D s_13_5: cast reint s_13_4 -> u64
        let s_13_5: u64 = (s_13_4.value() as u64);
        // D s_13_6: read-var dbytes:i64
        let s_13_6: i64 = fn_state.dbytes;
        // D s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // D s_13_8: read-var accdesc:struct
        let s_13_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_13_9: read-var data1:bv
        let s_13_9: Bits = fn_state.data1;
        // D s_13_10: call Mem_set(s_13_5, s_13_7, s_13_8, s_13_9)
        let s_13_10: () = Mem_set(state, tracer, s_13_5, s_13_7, s_13_8, s_13_9);
        // N s_13_11: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var address:u64
        let s_14_0: u64 = fn_state.address;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 64u16);
        // D s_14_2: read-var dbytes:i64
        let s_14_2: i64 = fn_state.dbytes;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cast cvt s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 128);
        // D s_14_5: add s_14_1 s_14_4
        let s_14_5: Bits = (s_14_1 + s_14_4);
        // D s_14_6: cast reint s_14_5 -> u64
        let s_14_6: u64 = (s_14_5.value() as u64);
        // D s_14_7: read-var dbytes:i64
        let s_14_7: i64 = fn_state.dbytes;
        // D s_14_8: cast zx s_14_7 -> i
        let s_14_8: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_9: read-var accdesc:struct
        let s_14_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_10: read-var data2:bv
        let s_14_10: Bits = fn_state.data2;
        // D s_14_11: call Mem_set(s_14_6, s_14_8, s_14_9, s_14_10)
        let s_14_11: () = Mem_set(state, tracer, s_14_6, s_14_8, s_14_9, s_14_10);
        // N s_14_12: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var wback:u8
        let s_15_0: bool = fn_state.wback;
        // N s_15_1: branch s_15_0 b20 b16
        if s_15_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #22416u : u32
        let s_17_0: u32 = 22416;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: bool = {
            let value = state.read_register::<bool>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // N s_17_2: branch s_17_1 b19 b18
        if s_17_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call SPESampleGeneralPurposeLoadStore(s_19_0)
        let s_19_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_19_0);
        // N s_19_2: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var postindex:u8
        let s_20_0: bool = fn_state.postindex;
        // N s_20_1: branch s_20_0 b25 b21
        if s_20_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #31s : i
        let s_22_0: i128 = 31;
        // D s_22_1: read-var n:i64
        let s_22_1: i64 = fn_state.n;
        // D s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (i128::try_from(s_22_1).unwrap());
        // D s_22_3: cmp-eq s_22_2 s_22_0
        let s_22_3: bool = ((s_22_2) == (s_22_0));
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // D s_23_1: read-var n:i64
        let s_23_1: i64 = fn_state.n;
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (i128::try_from(s_23_1).unwrap());
        // D s_23_3: read-var address:u64
        let s_23_3: u64 = fn_state.address;
        // D s_23_4: cast zx s_23_3 -> bv
        let s_23_4: Bits = Bits::new(s_23_3 as u128, 64u16);
        // D s_23_5: call X_set(s_23_2, s_23_0, s_23_4)
        let s_23_5: () = X_set(state, tracer, s_23_2, s_23_0, s_23_4);
        // N s_23_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var address:u64
        let s_24_0: u64 = fn_state.address;
        // D s_24_1: call SP_set(s_24_0)
        let s_24_1: () = SP_set(state, tracer, s_24_0);
        // N s_24_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var address:u64
        let s_25_0: u64 = fn_state.address;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 64u16);
        // D s_25_2: read-var offset:u64
        let s_25_2: u64 = fn_state.offset;
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 64u16);
        // D s_25_4: add s_25_1 s_25_3
        let s_25_4: Bits = (s_25_1 + s_25_3);
        // D s_25_5: cast reint s_25_4 -> u64
        let s_25_5: u64 = (s_25_4.value() as u64);
        // D s_25_6: write-var address <= s_25_5
        fn_state.address = s_25_5;
        // N s_25_7: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var datasizeshadow#1641:i64
        let s_26_0: i64 = fn_state.datasizeshadow_1641;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: cast reint s_26_1 -> i64
        let s_26_2: i64 = (s_26_1 as i64);
        // D s_26_3: cast zx s_26_2 -> i
        let s_26_3: i128 = (i128::try_from(s_26_2).unwrap());
        // D s_26_4: call __UNKNOWN_bits(s_26_3)
        let s_26_4: Bits = u__UNKNOWN_bits(state, tracer, s_26_3);
        // D s_26_5: write-var data2 <= s_26_4
        fn_state.data2 = s_26_4;
        // N s_26_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var t2:i64
        let s_27_0: i64 = fn_state.t2;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: read-var n:i64
        let s_27_2: i64 = fn_state.n;
        // D s_27_3: cast zx s_27_2 -> i
        let s_27_3: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#160526 <= s_27_4
        fn_state.gs_160526 = s_27_4;
        // N s_27_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var datasizeshadow#1641:i64
        let s_28_0: i64 = fn_state.datasizeshadow_1641;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: cast reint s_28_1 -> i64
        let s_28_2: i64 = (s_28_1 as i64);
        // D s_28_3: cast zx s_28_2 -> i
        let s_28_3: i128 = (i128::try_from(s_28_2).unwrap());
        // D s_28_4: call __UNKNOWN_bits(s_28_3)
        let s_28_4: Bits = u__UNKNOWN_bits(state, tracer, s_28_3);
        // D s_28_5: write-var data1 <= s_28_4
        fn_state.data1 = s_28_4;
        // N s_28_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var t:i64
        let s_29_0: i64 = fn_state.t;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: read-var n:i64
        let s_29_2: i64 = fn_state.n;
        // D s_29_3: cast zx s_29_2 -> i
        let s_29_3: i128 = (i128::try_from(s_29_2).unwrap());
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#160525 <= s_29_4
        fn_state.gs_160525 = s_29_4;
        // N s_29_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u32
        let s_30_0: u32 = 0;
        // D s_30_1: read-var memop:u32
        let s_30_1: u32 = fn_state.memop;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // N s_30_4: branch s_30_3 b43 b31
        if s_30_3 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call HaveLSE2Ext(s_31_0)
        let s_31_1: bool = HaveLSE2Ext(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b39 b32
        if s_31_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0s : i
        let s_32_0: i128 = 0;
        // D s_32_1: read-var address:u64
        let s_32_1: u64 = fn_state.address;
        // D s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 64u16);
        // C s_32_3: cast cvt s_32_0 -> bv
        let s_32_3: Bits = Bits::new(s_32_0 as u128, 128);
        // D s_32_4: add s_32_2 s_32_3
        let s_32_4: Bits = (s_32_2 + s_32_3);
        // D s_32_5: cast reint s_32_4 -> u64
        let s_32_5: u64 = (s_32_4.value() as u64);
        // D s_32_6: read-var dbytes:i64
        let s_32_6: i64 = fn_state.dbytes;
        // D s_32_7: cast zx s_32_6 -> i
        let s_32_7: i128 = (i128::try_from(s_32_6).unwrap());
        // D s_32_8: read-var accdesc:struct
        let s_32_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_32_9: call Mem_read(s_32_5, s_32_7, s_32_8)
        let s_32_9: Bits = Mem_read(state, tracer, s_32_5, s_32_7, s_32_8);
        // D s_32_10: write-var data1 <= s_32_9
        fn_state.data1 = s_32_9;
        // N s_32_11: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var address:u64
        let s_33_0: u64 = fn_state.address;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 64u16);
        // D s_33_2: read-var dbytes:i64
        let s_33_2: i64 = fn_state.dbytes;
        // D s_33_3: cast zx s_33_2 -> i
        let s_33_3: i128 = (i128::try_from(s_33_2).unwrap());
        // D s_33_4: cast cvt s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 128);
        // D s_33_5: add s_33_1 s_33_4
        let s_33_5: Bits = (s_33_1 + s_33_4);
        // D s_33_6: cast reint s_33_5 -> u64
        let s_33_6: u64 = (s_33_5.value() as u64);
        // D s_33_7: read-var dbytes:i64
        let s_33_7: i64 = fn_state.dbytes;
        // D s_33_8: cast zx s_33_7 -> i
        let s_33_8: i128 = (i128::try_from(s_33_7).unwrap());
        // D s_33_9: read-var accdesc:struct
        let s_33_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_33_10: call Mem_read(s_33_6, s_33_8, s_33_9)
        let s_33_10: Bits = Mem_read(state, tracer, s_33_6, s_33_8, s_33_9);
        // D s_33_11: write-var data2 <= s_33_10
        fn_state.data2 = s_33_10;
        // N s_33_12: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var rt_unknown:u8
        let s_35_0: bool = fn_state.rt_unknown;
        // N s_35_1: branch s_35_0 b38 b36
        if s_35_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var datasizeshadow#1641:i64
        let s_37_0: i64 = fn_state.datasizeshadow_1641;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: cast reint s_37_1 -> i64
        let s_37_2: i64 = (s_37_1 as i64);
        // D s_37_3: read-var t:i64
        let s_37_3: i64 = fn_state.t;
        // D s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_5: read-var data1:bv
        let s_37_5: Bits = fn_state.data1;
        // D s_37_6: call X_set(s_37_4, s_37_2, s_37_5)
        let s_37_6: () = X_set(state, tracer, s_37_4, s_37_2, s_37_5);
        // D s_37_7: read-var datasizeshadow#1641:i64
        let s_37_7: i64 = fn_state.datasizeshadow_1641;
        // D s_37_8: cast zx s_37_7 -> i
        let s_37_8: i128 = (i128::try_from(s_37_7).unwrap());
        // D s_37_9: cast reint s_37_8 -> i64
        let s_37_9: i64 = (s_37_8 as i64);
        // D s_37_10: read-var t2:i64
        let s_37_10: i64 = fn_state.t2;
        // D s_37_11: cast zx s_37_10 -> i
        let s_37_11: i128 = (i128::try_from(s_37_10).unwrap());
        // D s_37_12: read-var data2:bv
        let s_37_12: Bits = fn_state.data2;
        // D s_37_13: call X_set(s_37_11, s_37_9, s_37_12)
        let s_37_13: () = X_set(state, tracer, s_37_11, s_37_9, s_37_12);
        // N s_37_14: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var datasizeshadow#1641:i64
        let s_38_0: i64 = fn_state.datasizeshadow_1641;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: cast reint s_38_1 -> i64
        let s_38_2: i64 = (s_38_1 as i64);
        // D s_38_3: cast zx s_38_2 -> i
        let s_38_3: i128 = (i128::try_from(s_38_2).unwrap());
        // D s_38_4: call __UNKNOWN_bits(s_38_3)
        let s_38_4: Bits = u__UNKNOWN_bits(state, tracer, s_38_3);
        // D s_38_5: write-var data1 <= s_38_4
        fn_state.data1 = s_38_4;
        // D s_38_6: read-var datasizeshadow#1641:i64
        let s_38_6: i64 = fn_state.datasizeshadow_1641;
        // D s_38_7: cast zx s_38_6 -> i
        let s_38_7: i128 = (i128::try_from(s_38_6).unwrap());
        // D s_38_8: cast reint s_38_7 -> i64
        let s_38_8: i64 = (s_38_7 as i64);
        // D s_38_9: cast zx s_38_8 -> i
        let s_38_9: i128 = (i128::try_from(s_38_8).unwrap());
        // D s_38_10: call __UNKNOWN_bits(s_38_9)
        let s_38_10: Bits = u__UNKNOWN_bits(state, tracer, s_38_9);
        // D s_38_11: write-var data2 <= s_38_10
        fn_state.data2 = s_38_10;
        // N s_38_12: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #2s : i
        let s_39_0: i128 = 2;
        // D s_39_1: read-var dbytes:i64
        let s_39_1: i64 = fn_state.dbytes;
        // D s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (i128::try_from(s_39_1).unwrap());
        // D s_39_3: mul s_39_0 s_39_2
        let s_39_3: i128 = ((s_39_0) * (s_39_2));
        // D s_39_4: cast reint s_39_3 -> i64
        let s_39_4: i64 = (s_39_3 as i64);
        // D s_39_5: cast zx s_39_4 -> i
        let s_39_5: i128 = (i128::try_from(s_39_4).unwrap());
        // D s_39_6: read-var address:u64
        let s_39_6: u64 = fn_state.address;
        // D s_39_7: read-var accdesc:struct
        let s_39_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // C s_39_8: const #1u : u8
        let s_39_8: bool = true;
        // D s_39_9: call Mem_read__1(s_39_6, s_39_5, s_39_7, s_39_8)
        let s_39_9: Bits = Mem_read__1(state, tracer, s_39_6, s_39_5, s_39_7, s_39_8);
        // D s_39_10: write-var full_data <= s_39_9
        fn_state.full_data = s_39_9;
        // N s_39_11: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var accdesc.1:struct
        let s_40_0: u32 = fn_state.accdesc._1;
        // D s_40_1: call BigEndian(s_40_0)
        let s_40_1: bool = BigEndian(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b42 b41
        if s_40_1 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1s : i
        let s_41_0: i128 = 1;
        // D s_41_1: read-var datasizeshadow#1641:i64
        let s_41_1: i64 = fn_state.datasizeshadow_1641;
        // D s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (i128::try_from(s_41_1).unwrap());
        // D s_41_3: sub s_41_2 s_41_0
        let s_41_3: i128 = ((s_41_2) - (s_41_0));
        // D s_41_4: cast reint s_41_3 -> i64
        let s_41_4: i64 = (s_41_3 as i64);
        // C s_41_5: const #0s : i
        let s_41_5: i128 = 0;
        // D s_41_6: cast zx s_41_4 -> i
        let s_41_6: i128 = (i128::try_from(s_41_4).unwrap());
        // D s_41_7: read-var full_data:bv
        let s_41_7: Bits = fn_state.full_data;
        // C s_41_8: const #1s : i64
        let s_41_8: i64 = 1;
        // C s_41_9: cast zx s_41_8 -> i
        let s_41_9: i128 = (i128::try_from(s_41_8).unwrap());
        // D s_41_10: sub s_41_6 s_41_5
        let s_41_10: i128 = ((s_41_6) - (s_41_5));
        // D s_41_11: add s_41_10 s_41_9
        let s_41_11: i128 = (s_41_10 + s_41_9);
        // D s_41_12: bit-extract s_41_7 s_41_5 s_41_11
        let s_41_12: Bits = (Bits::new(
            ((s_41_7) >> (s_41_5)).value(),
            u16::try_from(s_41_11).unwrap(),
        ));
        // D s_41_13: write-var data1 <= s_41_12
        fn_state.data1 = s_41_12;
        // C s_41_14: const #2s : i
        let s_41_14: i128 = 2;
        // D s_41_15: read-var datasizeshadow#1641:i64
        let s_41_15: i64 = fn_state.datasizeshadow_1641;
        // D s_41_16: cast zx s_41_15 -> i
        let s_41_16: i128 = (i128::try_from(s_41_15).unwrap());
        // D s_41_17: mul s_41_14 s_41_16
        let s_41_17: i128 = ((s_41_14) * (s_41_16));
        // D s_41_18: cast reint s_41_17 -> i64
        let s_41_18: i64 = (s_41_17 as i64);
        // C s_41_19: const #1s : i
        let s_41_19: i128 = 1;
        // D s_41_20: cast zx s_41_18 -> i
        let s_41_20: i128 = (i128::try_from(s_41_18).unwrap());
        // D s_41_21: sub s_41_20 s_41_19
        let s_41_21: i128 = ((s_41_20) - (s_41_19));
        // D s_41_22: cast reint s_41_21 -> i64
        let s_41_22: i64 = (s_41_21 as i64);
        // D s_41_23: cast zx s_41_22 -> i
        let s_41_23: i128 = (i128::try_from(s_41_22).unwrap());
        // D s_41_24: read-var datasizeshadow#1641:i64
        let s_41_24: i64 = fn_state.datasizeshadow_1641;
        // D s_41_25: cast zx s_41_24 -> i
        let s_41_25: i128 = (i128::try_from(s_41_24).unwrap());
        // D s_41_26: read-var full_data:bv
        let s_41_26: Bits = fn_state.full_data;
        // C s_41_27: const #1s : i64
        let s_41_27: i64 = 1;
        // C s_41_28: cast zx s_41_27 -> i
        let s_41_28: i128 = (i128::try_from(s_41_27).unwrap());
        // D s_41_29: sub s_41_23 s_41_25
        let s_41_29: i128 = ((s_41_23) - (s_41_25));
        // D s_41_30: add s_41_29 s_41_28
        let s_41_30: i128 = (s_41_29 + s_41_28);
        // D s_41_31: bit-extract s_41_26 s_41_25 s_41_30
        let s_41_31: Bits = (Bits::new(
            ((s_41_26) >> (s_41_25)).value(),
            u16::try_from(s_41_30).unwrap(),
        ));
        // D s_41_32: write-var data2 <= s_41_31
        fn_state.data2 = s_41_31;
        // N s_41_33: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1s : i
        let s_42_0: i128 = 1;
        // D s_42_1: read-var datasizeshadow#1641:i64
        let s_42_1: i64 = fn_state.datasizeshadow_1641;
        // D s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (i128::try_from(s_42_1).unwrap());
        // D s_42_3: sub s_42_2 s_42_0
        let s_42_3: i128 = ((s_42_2) - (s_42_0));
        // D s_42_4: cast reint s_42_3 -> i64
        let s_42_4: i64 = (s_42_3 as i64);
        // C s_42_5: const #0s : i
        let s_42_5: i128 = 0;
        // D s_42_6: cast zx s_42_4 -> i
        let s_42_6: i128 = (i128::try_from(s_42_4).unwrap());
        // D s_42_7: read-var full_data:bv
        let s_42_7: Bits = fn_state.full_data;
        // C s_42_8: const #1s : i64
        let s_42_8: i64 = 1;
        // C s_42_9: cast zx s_42_8 -> i
        let s_42_9: i128 = (i128::try_from(s_42_8).unwrap());
        // D s_42_10: sub s_42_6 s_42_5
        let s_42_10: i128 = ((s_42_6) - (s_42_5));
        // D s_42_11: add s_42_10 s_42_9
        let s_42_11: i128 = (s_42_10 + s_42_9);
        // D s_42_12: bit-extract s_42_7 s_42_5 s_42_11
        let s_42_12: Bits = (Bits::new(
            ((s_42_7) >> (s_42_5)).value(),
            u16::try_from(s_42_11).unwrap(),
        ));
        // D s_42_13: write-var data2 <= s_42_12
        fn_state.data2 = s_42_12;
        // C s_42_14: const #2s : i
        let s_42_14: i128 = 2;
        // D s_42_15: read-var datasizeshadow#1641:i64
        let s_42_15: i64 = fn_state.datasizeshadow_1641;
        // D s_42_16: cast zx s_42_15 -> i
        let s_42_16: i128 = (i128::try_from(s_42_15).unwrap());
        // D s_42_17: mul s_42_14 s_42_16
        let s_42_17: i128 = ((s_42_14) * (s_42_16));
        // D s_42_18: cast reint s_42_17 -> i64
        let s_42_18: i64 = (s_42_17 as i64);
        // C s_42_19: const #1s : i
        let s_42_19: i128 = 1;
        // D s_42_20: cast zx s_42_18 -> i
        let s_42_20: i128 = (i128::try_from(s_42_18).unwrap());
        // D s_42_21: sub s_42_20 s_42_19
        let s_42_21: i128 = ((s_42_20) - (s_42_19));
        // D s_42_22: cast reint s_42_21 -> i64
        let s_42_22: i64 = (s_42_21 as i64);
        // D s_42_23: cast zx s_42_22 -> i
        let s_42_23: i128 = (i128::try_from(s_42_22).unwrap());
        // D s_42_24: read-var datasizeshadow#1641:i64
        let s_42_24: i64 = fn_state.datasizeshadow_1641;
        // D s_42_25: cast zx s_42_24 -> i
        let s_42_25: i128 = (i128::try_from(s_42_24).unwrap());
        // D s_42_26: read-var full_data:bv
        let s_42_26: Bits = fn_state.full_data;
        // C s_42_27: const #1s : i64
        let s_42_27: i64 = 1;
        // C s_42_28: cast zx s_42_27 -> i
        let s_42_28: i128 = (i128::try_from(s_42_27).unwrap());
        // D s_42_29: sub s_42_23 s_42_25
        let s_42_29: i128 = ((s_42_23) - (s_42_25));
        // D s_42_30: add s_42_29 s_42_28
        let s_42_30: i128 = (s_42_29 + s_42_28);
        // D s_42_31: bit-extract s_42_26 s_42_25 s_42_30
        let s_42_31: Bits = (Bits::new(
            ((s_42_26) >> (s_42_25)).value(),
            u16::try_from(s_42_30).unwrap(),
        ));
        // D s_42_32: write-var data1 <= s_42_31
        fn_state.data1 = s_42_31;
        // N s_42_33: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var address:u64
        let s_44_0: u64 = fn_state.address;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 64u16);
        // D s_44_2: read-var offset:u64
        let s_44_2: u64 = fn_state.offset;
        // D s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 64u16);
        // D s_44_4: add s_44_1 s_44_3
        let s_44_4: Bits = (s_44_1 + s_44_3);
        // D s_44_5: cast reint s_44_4 -> u64
        let s_44_5: u64 = (s_44_4.value() as u64);
        // D s_44_6: write-var address <= s_44_5
        fn_state.address = s_44_5;
        // N s_44_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call CheckSPAlignment(s_45_0)
        let s_45_1: () = CheckSPAlignment(state, tracer, s_45_0);
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call SP_read(s_46_0)
        let s_46_1: u64 = SP_read(state, tracer, s_46_0);
        // D s_46_2: write-var address <= s_46_1
        fn_state.address = s_46_1;
        // N s_46_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
