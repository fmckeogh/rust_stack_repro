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
use CheckFPEnabled64::*;
use V_set::*;
use u__id::*;
use V_read::*;
use SP_set::*;
use Mem_read::*;
use u__UNKNOWN_bits::*;
use SP_read::*;
use SPESampleSIMDFPLoadStore::*;
use X_read::*;
use CreateAccDescASIMD::*;
use CheckSPAlignment::*;
use Mem_set::*;
use common::*;
pub fn execute_aarch64_instrs_memory_pair_simdfp_no_alloc<T: Tracer>(
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
        data2shadow_1644: Bits,
        gs_160651: bool,
        address: u64,
        gs_160657: bool,
        gs_160641: bool,
        gs_160639: bool,
        data2: Bits,
        datasizeshadow_1642: i64,
        gs_160637: bool,
        dbytes: i64,
        gs_160655: bool,
        accdesc: ProductType9878976b5bcce9c9,
        data1: Bits,
        gs_160643: bool,
        gs_160653: bool,
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
        // D s_0_3: write-var datasizeshadow#1642 <= s_0_2
        fn_state.datasizeshadow_1642 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPEnabled64(s_0_4)
        let s_0_5: () = CheckFPEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #8s : i
        let s_1_0: i128 = 8;
        // D s_1_1: read-var datasizeshadow#1642:i64
        let s_1_1: i64 = fn_state.datasizeshadow_1642;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: div s_1_2 s_1_0
        let s_1_3: i128 = ((s_1_2) / (s_1_0));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var dbytes <= s_1_4
        fn_state.dbytes = s_1_4;
        // D s_1_6: read-var memop:u32
        let s_1_6: u32 = fn_state.memop;
        // D s_1_7: read-var nontemporal:u8
        let s_1_7: bool = fn_state.nontemporal;
        // D s_1_8: read-var tagchecked:u8
        let s_1_8: bool = fn_state.tagchecked;
        // D s_1_9: call CreateAccDescASIMD(s_1_6, s_1_7, s_1_8)
        let s_1_9: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_6,
            s_1_7,
            s_1_8,
        );
        // D s_1_10: write-var accdesc <= s_1_9
        fn_state.accdesc = s_1_9;
        // C s_1_11: const #31s : i
        let s_1_11: i128 = 31;
        // D s_1_12: read-var n:i64
        let s_1_12: i64 = fn_state.n;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cmp-eq s_1_13 s_1_11
        let s_1_14: bool = ((s_1_13) == (s_1_11));
        // N s_1_15: branch s_1_14 b52 b2
        if s_1_14 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var address <= s_2_4
        fn_state.address = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var postindex:u8
        let s_3_0: bool = fn_state.postindex;
        // D s_3_1: not s_3_0
        let s_3_1: bool = !s_3_0;
        // N s_3_2: branch s_3_1 b51 b4
        if s_3_1 {
            return block_51(state, tracer, fn_state);
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
        // C s_5_0: const #1u : u32
        let s_5_0: u32 = 1;
        // D s_5_1: read-var memop:u32
        let s_5_1: u32 = fn_state.memop;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b31 b6
        if s_5_3 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var datasizeshadow#1642:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1642;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #8s : i
        let s_6_4: i128 = 8;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // N s_6_7: branch s_6_6 b30 b7
        if s_6_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#1642:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1642;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #16s : i
        let s_7_4: i128 = 16;
        // D s_7_5: cast zx s_7_3 -> i
        let s_7_5: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_6: cmp-eq s_7_5 s_7_4
        let s_7_6: bool = ((s_7_5) == (s_7_4));
        // D s_7_7: write-var gs#160637 <= s_7_6
        fn_state.gs_160637 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#160637:u8
        let s_8_0: bool = fn_state.gs_160637;
        // N s_8_1: branch s_8_0 b29 b9
        if s_8_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var datasizeshadow#1642:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1642;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #32s : i
        let s_9_4: i128 = 32;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // D s_9_7: write-var gs#160639 <= s_9_6
        fn_state.gs_160639 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#160639:u8
        let s_10_0: bool = fn_state.gs_160639;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var datasizeshadow#1642:i64
        let s_11_0: i64 = fn_state.datasizeshadow_1642;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call __id(s_11_1)
        let s_11_2: i128 = u__id(state, tracer, s_11_1);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #64s : i
        let s_11_4: i128 = 64;
        // D s_11_5: cast zx s_11_3 -> i
        let s_11_5: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_6: cmp-eq s_11_5 s_11_4
        let s_11_6: bool = ((s_11_5) == (s_11_4));
        // D s_11_7: write-var gs#160641 <= s_11_6
        fn_state.gs_160641 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#160641:u8
        let s_12_0: bool = fn_state.gs_160641;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var datasizeshadow#1642:i64
        let s_13_0: i64 = fn_state.datasizeshadow_1642;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #128s : i
        let s_13_4: i128 = 128;
        // D s_13_5: cast zx s_13_3 -> i
        let s_13_5: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_6: cmp-eq s_13_5 s_13_4
        let s_13_6: bool = ((s_13_5) == (s_13_4));
        // D s_13_7: write-var gs#160643 <= s_13_6
        fn_state.gs_160643 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#160643:u8
        let s_14_0: bool = fn_state.gs_160643;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var datasizeshadow#1642:i64
        let s_14_2: i64 = fn_state.datasizeshadow_1642;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // D s_14_5: read-var t:i64
        let s_14_5: i64 = fn_state.t;
        // D s_14_6: cast zx s_14_5 -> i
        let s_14_6: i128 = (i128::try_from(s_14_5).unwrap());
        // D s_14_7: call V_read(s_14_6, s_14_4)
        let s_14_7: Bits = V_read(state, tracer, s_14_6, s_14_4);
        // D s_14_8: read-var datasizeshadow#1642:i64
        let s_14_8: i64 = fn_state.datasizeshadow_1642;
        // D s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // D s_14_10: cast reint s_14_9 -> i64
        let s_14_10: i64 = (s_14_9 as i64);
        // D s_14_11: read-var t2:i64
        let s_14_11: i64 = fn_state.t2;
        // D s_14_12: cast zx s_14_11 -> i
        let s_14_12: i128 = (i128::try_from(s_14_11).unwrap());
        // D s_14_13: call V_read(s_14_12, s_14_10)
        let s_14_13: Bits = V_read(state, tracer, s_14_12, s_14_10);
        // D s_14_14: write-var data2shadow#1644 <= s_14_13
        fn_state.data2shadow_1644 = s_14_13;
        // C s_14_15: const #0s : i
        let s_14_15: i128 = 0;
        // D s_14_16: read-var address:u64
        let s_14_16: u64 = fn_state.address;
        // D s_14_17: cast zx s_14_16 -> bv
        let s_14_17: Bits = Bits::new(s_14_16 as u128, 64u16);
        // C s_14_18: cast cvt s_14_15 -> bv
        let s_14_18: Bits = Bits::new(s_14_15 as u128, 128);
        // D s_14_19: add s_14_17 s_14_18
        let s_14_19: Bits = (s_14_17 + s_14_18);
        // D s_14_20: cast reint s_14_19 -> u64
        let s_14_20: u64 = (s_14_19.value() as u64);
        // D s_14_21: read-var dbytes:i64
        let s_14_21: i64 = fn_state.dbytes;
        // D s_14_22: cast zx s_14_21 -> i
        let s_14_22: i128 = (i128::try_from(s_14_21).unwrap());
        // D s_14_23: read-var accdesc:struct
        let s_14_23: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_24: call Mem_set(s_14_20, s_14_22, s_14_23, s_14_7)
        let s_14_24: () = Mem_set(state, tracer, s_14_20, s_14_22, s_14_23, s_14_7);
        // N s_14_25: jump b15
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
        // D s_15_10: read-var data2shadow#1644:bv
        let s_15_10: Bits = fn_state.data2shadow_1644;
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
        // S s_20_1: call SPESampleSIMDFPLoadStore(s_20_0)
        let s_20_1: () = SPESampleSIMDFPLoadStore(state, tracer, s_20_0);
        // N s_20_2: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var postindex:u8
        let s_21_0: bool = fn_state.postindex;
        // N s_21_1: branch s_21_0 b26 b22
        if s_21_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #31s : i
        let s_23_0: i128 = 31;
        // D s_23_1: read-var n:i64
        let s_23_1: i64 = fn_state.n;
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (i128::try_from(s_23_1).unwrap());
        // D s_23_3: cmp-eq s_23_2 s_23_0
        let s_23_3: bool = ((s_23_2) == (s_23_0));
        // N s_23_4: branch s_23_3 b25 b24
        if s_23_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // D s_24_1: read-var n:i64
        let s_24_1: i64 = fn_state.n;
        // D s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // D s_24_3: read-var address:u64
        let s_24_3: u64 = fn_state.address;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 64u16);
        // D s_24_5: call X_set(s_24_2, s_24_0, s_24_4)
        let s_24_5: () = X_set(state, tracer, s_24_2, s_24_0, s_24_4);
        // N s_24_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var address:u64
        let s_25_0: u64 = fn_state.address;
        // D s_25_1: call SP_set(s_25_0)
        let s_25_1: () = SP_set(state, tracer, s_25_0);
        // N s_25_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var address:u64
        let s_26_0: u64 = fn_state.address;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 64u16);
        // D s_26_2: read-var offset:u64
        let s_26_2: u64 = fn_state.offset;
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 64u16);
        // D s_26_4: add s_26_1 s_26_3
        let s_26_4: Bits = (s_26_1 + s_26_3);
        // D s_26_5: cast reint s_26_4 -> u64
        let s_26_5: u64 = (s_26_4.value() as u64);
        // D s_26_6: write-var address <= s_26_5
        fn_state.address = s_26_5;
        // N s_26_7: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#160643 <= s_27_0
        fn_state.gs_160643 = s_27_0;
        // N s_27_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#160641 <= s_28_0
        fn_state.gs_160641 = s_28_0;
        // N s_28_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#160639 <= s_29_0
        fn_state.gs_160639 = s_29_0;
        // N s_29_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#160637 <= s_30_0
        fn_state.gs_160637 = s_30_0;
        // N s_30_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u32
        let s_31_0: u32 = 0;
        // D s_31_1: read-var memop:u32
        let s_31_1: u32 = fn_state.memop;
        // D s_31_2: cmp-eq s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) == (s_31_1));
        // D s_31_3: not s_31_2
        let s_31_3: bool = !s_31_2;
        // N s_31_4: branch s_31_3 b50 b32
        if s_31_3 {
            return block_50(state, tracer, fn_state);
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
        // D s_34_0: read-var rt_unknown:u8
        let s_34_0: bool = fn_state.rt_unknown;
        // N s_34_1: branch s_34_0 b49 b35
        if s_34_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var datasizeshadow#1642:i64
        let s_36_0: i64 = fn_state.datasizeshadow_1642;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call __id(s_36_1)
        let s_36_2: i128 = u__id(state, tracer, s_36_1);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #8s : i
        let s_36_4: i128 = 8;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-eq s_36_5 s_36_4
        let s_36_6: bool = ((s_36_5) == (s_36_4));
        // N s_36_7: branch s_36_6 b48 b37
        if s_36_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var datasizeshadow#1642:i64
        let s_37_0: i64 = fn_state.datasizeshadow_1642;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: call __id(s_37_1)
        let s_37_2: i128 = u__id(state, tracer, s_37_1);
        // D s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: const #16s : i
        let s_37_4: i128 = 16;
        // D s_37_5: cast zx s_37_3 -> i
        let s_37_5: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_6: cmp-eq s_37_5 s_37_4
        let s_37_6: bool = ((s_37_5) == (s_37_4));
        // D s_37_7: write-var gs#160651 <= s_37_6
        fn_state.gs_160651 = s_37_6;
        // N s_37_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#160651:u8
        let s_38_0: bool = fn_state.gs_160651;
        // N s_38_1: branch s_38_0 b47 b39
        if s_38_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var datasizeshadow#1642:i64
        let s_39_0: i64 = fn_state.datasizeshadow_1642;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call __id(s_39_1)
        let s_39_2: i128 = u__id(state, tracer, s_39_1);
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #32s : i
        let s_39_4: i128 = 32;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: cmp-eq s_39_5 s_39_4
        let s_39_6: bool = ((s_39_5) == (s_39_4));
        // D s_39_7: write-var gs#160653 <= s_39_6
        fn_state.gs_160653 = s_39_6;
        // N s_39_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#160653:u8
        let s_40_0: bool = fn_state.gs_160653;
        // N s_40_1: branch s_40_0 b46 b41
        if s_40_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var datasizeshadow#1642:i64
        let s_41_0: i64 = fn_state.datasizeshadow_1642;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: call __id(s_41_1)
        let s_41_2: i128 = u__id(state, tracer, s_41_1);
        // D s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #64s : i
        let s_41_4: i128 = 64;
        // D s_41_5: cast zx s_41_3 -> i
        let s_41_5: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_6: cmp-eq s_41_5 s_41_4
        let s_41_6: bool = ((s_41_5) == (s_41_4));
        // D s_41_7: write-var gs#160655 <= s_41_6
        fn_state.gs_160655 = s_41_6;
        // N s_41_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#160655:u8
        let s_42_0: bool = fn_state.gs_160655;
        // N s_42_1: branch s_42_0 b45 b43
        if s_42_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var datasizeshadow#1642:i64
        let s_43_0: i64 = fn_state.datasizeshadow_1642;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: call __id(s_43_1)
        let s_43_2: i128 = u__id(state, tracer, s_43_1);
        // D s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: const #128s : i
        let s_43_4: i128 = 128;
        // D s_43_5: cast zx s_43_3 -> i
        let s_43_5: i128 = (i128::try_from(s_43_3).unwrap());
        // D s_43_6: cmp-eq s_43_5 s_43_4
        let s_43_6: bool = ((s_43_5) == (s_43_4));
        // D s_43_7: write-var gs#160657 <= s_43_6
        fn_state.gs_160657 = s_43_6;
        // N s_43_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#160657:u8
        let s_44_0: bool = fn_state.gs_160657;
        // N s_44_1: assert s_44_0
        let s_44_1: () = assert!(s_44_0);
        // D s_44_2: read-var datasizeshadow#1642:i64
        let s_44_2: i64 = fn_state.datasizeshadow_1642;
        // D s_44_3: cast zx s_44_2 -> i
        let s_44_3: i128 = (i128::try_from(s_44_2).unwrap());
        // D s_44_4: cast reint s_44_3 -> i64
        let s_44_4: i64 = (s_44_3 as i64);
        // D s_44_5: read-var t:i64
        let s_44_5: i64 = fn_state.t;
        // D s_44_6: cast zx s_44_5 -> i
        let s_44_6: i128 = (i128::try_from(s_44_5).unwrap());
        // D s_44_7: read-var data1:bv
        let s_44_7: Bits = fn_state.data1;
        // D s_44_8: call V_set(s_44_6, s_44_4, s_44_7)
        let s_44_8: () = V_set(state, tracer, s_44_6, s_44_4, s_44_7);
        // D s_44_9: read-var datasizeshadow#1642:i64
        let s_44_9: i64 = fn_state.datasizeshadow_1642;
        // D s_44_10: cast zx s_44_9 -> i
        let s_44_10: i128 = (i128::try_from(s_44_9).unwrap());
        // D s_44_11: cast reint s_44_10 -> i64
        let s_44_11: i64 = (s_44_10 as i64);
        // D s_44_12: read-var t2:i64
        let s_44_12: i64 = fn_state.t2;
        // D s_44_13: cast zx s_44_12 -> i
        let s_44_13: i128 = (i128::try_from(s_44_12).unwrap());
        // D s_44_14: read-var data2:bv
        let s_44_14: Bits = fn_state.data2;
        // D s_44_15: call V_set(s_44_13, s_44_11, s_44_14)
        let s_44_15: () = V_set(state, tracer, s_44_13, s_44_11, s_44_14);
        // N s_44_16: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#160657 <= s_45_0
        fn_state.gs_160657 = s_45_0;
        // N s_45_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#160655 <= s_46_0
        fn_state.gs_160655 = s_46_0;
        // N s_46_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#160653 <= s_47_0
        fn_state.gs_160653 = s_47_0;
        // N s_47_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#160651 <= s_48_0
        fn_state.gs_160651 = s_48_0;
        // N s_48_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var datasizeshadow#1642:i64
        let s_49_0: i64 = fn_state.datasizeshadow_1642;
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
        // D s_49_6: read-var datasizeshadow#1642:i64
        let s_49_6: i64 = fn_state.datasizeshadow_1642;
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
        // N s_49_12: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var address:u64
        let s_51_0: u64 = fn_state.address;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 64u16);
        // D s_51_2: read-var offset:u64
        let s_51_2: u64 = fn_state.offset;
        // D s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 64u16);
        // D s_51_4: add s_51_1 s_51_3
        let s_51_4: Bits = (s_51_1 + s_51_3);
        // D s_51_5: cast reint s_51_4 -> u64
        let s_51_5: u64 = (s_51_4.value() as u64);
        // D s_51_6: write-var address <= s_51_5
        fn_state.address = s_51_5;
        // N s_51_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call CheckSPAlignment(s_52_0)
        let s_52_1: () = CheckSPAlignment(state, tracer, s_52_0);
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call SP_read(s_53_0)
        let s_53_1: u64 = SP_read(state, tracer, s_53_0);
        // D s_53_2: write-var address <= s_53_1
        fn_state.address = s_53_1;
        // N s_53_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
