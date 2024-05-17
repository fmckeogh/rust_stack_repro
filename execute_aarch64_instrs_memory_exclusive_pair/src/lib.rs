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
use SPESetDataVirtualAddress::*;
use BigEndian::*;
use IsAligned__1::*;
use X_read::*;
use AArch64_ExclusiveMonitorsPass::*;
use X_set::*;
use Mem_set::*;
use u__UNKNOWN_bits::*;
use u__id::*;
use AArch64_SetExclusiveMonitors::*;
use CreateAccDescExLDST::*;
use Mem_read::*;
use AArch64_Abort::*;
use SP_read::*;
use CheckSPAlignment::*;
use AlignmentFault::*;
use SPESampleExtendedLoadStore::*;
use common::*;
pub fn execute_aarch64_instrs_memory_exclusive_pair<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acqrel: bool,
    datasize: i64,
    elsize: i64,
    memop: u32,
    n: i64,
    pair: bool,
    regsize: i64,
    rn_unknown: bool,
    rt_unknown: bool,
    s: i64,
    t: i64,
    t2: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_159457: bool,
        gs_701903: Bits,
        address: u64,
        datashadow_1630: Bits,
        el2: Bits,
        ar: bool,
        gs_159402: bool,
        gs_159418: bool,
        gs_159435: bool,
        dbytes: i64,
        gs_159404: bool,
        gs_159451: bool,
        gs_701909: Bits,
        ga_259397: bool,
        el1: Bits,
        datashadow_1631: Bits,
        gs_159469: bool,
        datasizeshadow_1629: i64,
        data: Bits,
        status: bool,
        accdesc: ProductType9878976b5bcce9c9,
        gs_159406: bool,
        regsizeshadow_1627: i64,
        elsizeshadow_1628: i64,
        gs_159441: bool,
        acqrel: bool,
        datasize: i64,
        elsize: i64,
        memop: u32,
        n: i64,
        pair: bool,
        regsize: i64,
        rn_unknown: bool,
        rt_unknown: bool,
        s: i64,
        t: i64,
        t2: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        acqrel,
        datasize,
        elsize,
        memop,
        n,
        pair,
        regsize,
        rn_unknown,
        rt_unknown,
        s,
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
        // D s_0_0: read-var regsize:i64
        let s_0_0: i64 = fn_state.regsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var regsizeshadow#1627 <= s_0_2
        fn_state.regsizeshadow_1627 = s_0_2;
        // D s_0_4: read-var elsize:i64
        let s_0_4: i64 = fn_state.elsize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var elsizeshadow#1628 <= s_0_6
        fn_state.elsizeshadow_1628 = s_0_6;
        // D s_0_8: read-var datasize:i64
        let s_0_8: i64 = fn_state.datasize;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var datasizeshadow#1629 <= s_0_10
        fn_state.datasizeshadow_1629 = s_0_10;
        // C s_0_12: const #8s : i
        let s_0_12: i128 = 8;
        // D s_0_13: read-var datasizeshadow#1629:i64
        let s_0_13: i64 = fn_state.datasizeshadow_1629;
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: div s_0_14 s_0_12
        let s_0_15: i128 = ((s_0_14) / (s_0_12));
        // D s_0_16: cast reint s_0_15 -> i64
        let s_0_16: i64 = (s_0_15 as i64);
        // D s_0_17: write-var dbytes <= s_0_16
        fn_state.dbytes = s_0_16;
        // D s_0_18: read-var memop:u32
        let s_0_18: u32 = fn_state.memop;
        // D s_0_19: read-var acqrel:u8
        let s_0_19: bool = fn_state.acqrel;
        // D s_0_20: read-var tagchecked:u8
        let s_0_20: bool = fn_state.tagchecked;
        // D s_0_21: call CreateAccDescExLDST(s_0_18, s_0_19, s_0_20)
        let s_0_21: ProductType9878976b5bcce9c9 = CreateAccDescExLDST(
            state,
            tracer,
            s_0_18,
            s_0_19,
            s_0_20,
        );
        // D s_0_22: write-var accdesc <= s_0_21
        fn_state.accdesc = s_0_21;
        // C s_0_23: const #31s : i
        let s_0_23: i128 = 31;
        // D s_0_24: read-var n:i64
        let s_0_24: i64 = fn_state.n;
        // D s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_26: cmp-eq s_0_25 s_0_23
        let s_0_26: bool = ((s_0_25) == (s_0_23));
        // N s_0_27: branch s_0_26 b73 b1
        if s_0_26 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var rn_unknown:u8
        let s_1_0: bool = fn_state.rn_unknown;
        // N s_1_1: branch s_1_0 b72 b2
        if s_1_0 {
            return block_72(state, tracer, fn_state);
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
        // C s_3_0: const #1u : u32
        let s_3_0: u32 = 1;
        // D s_3_1: read-var memop:u32
        let s_3_1: u32 = fn_state.memop;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b35 b4
        if s_3_3 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var rt_unknown:u8
        let s_4_0: bool = fn_state.rt_unknown;
        // N s_4_1: branch s_4_0 b34 b5
        if s_4_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var pair:u8
        let s_5_0: bool = fn_state.pair;
        // N s_5_1: branch s_5_0 b30 b6
        if s_5_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var datasizeshadow#1629:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1629;
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
        // N s_6_7: branch s_6_6 b29 b7
        if s_6_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#1629:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1629;
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
        // D s_7_7: write-var gs#159402 <= s_7_6
        fn_state.gs_159402 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#159402:u8
        let s_8_0: bool = fn_state.gs_159402;
        // N s_8_1: branch s_8_0 b28 b9
        if s_8_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var datasizeshadow#1629:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1629;
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
        // D s_9_7: write-var gs#159404 <= s_9_6
        fn_state.gs_159404 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#159404:u8
        let s_10_0: bool = fn_state.gs_159404;
        // N s_10_1: branch s_10_0 b27 b11
        if s_10_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var datasizeshadow#1629:i64
        let s_11_0: i64 = fn_state.datasizeshadow_1629;
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
        // D s_11_7: write-var gs#159406 <= s_11_6
        fn_state.gs_159406 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#159406:u8
        let s_12_0: bool = fn_state.gs_159406;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var datasizeshadow#1629:i64
        let s_12_2: i64 = fn_state.datasizeshadow_1629;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: read-var t:i64
        let s_12_5: i64 = fn_state.t;
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: call X_read(s_12_6, s_12_4)
        let s_12_7: Bits = X_read(state, tracer, s_12_6, s_12_4);
        // D s_12_8: write-var data <= s_12_7
        fn_state.data = s_12_7;
        // N s_12_9: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var status <= s_13_0
        fn_state.status = s_13_0;
        // D s_13_2: read-var dbytes:i64
        let s_13_2: i64 = fn_state.dbytes;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: read-var address:u64
        let s_13_4: u64 = fn_state.address;
        // D s_13_5: call AArch64_ExclusiveMonitorsPass(s_13_4, s_13_3)
        let s_13_5: bool = AArch64_ExclusiveMonitorsPass(state, tracer, s_13_4, s_13_3);
        // D s_13_6: write-var ga#259397 <= s_13_5
        fn_state.ga_259397 = s_13_5;
        // N s_13_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#259397:u8
        let s_14_0: bool = fn_state.ga_259397;
        // N s_14_1: branch s_14_0 b25 b15
        if s_14_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #22416u : u32
        let s_15_0: u32 = 22416;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: bool = {
            let value = state.read_register::<bool>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // N s_15_2: branch s_15_1 b24 b16
        if s_15_1 {
            return block_24(state, tracer, fn_state);
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
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // C s_17_1: const #32s : i
        let s_17_1: i128 = 32;
        // D s_17_2: read-var status:u8
        let s_17_2: bool = fn_state.status;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: bits-cast zx s_17_3 -> bv length s_17_1
        let s_17_4: Bits = s_17_3.zero_extend(s_17_1);
        // D s_17_5: cast reint s_17_4 -> u32
        let s_17_5: u32 = (s_17_4.value() as u32);
        // D s_17_6: read-var s:i64
        let s_17_6: i64 = fn_state.s;
        // D s_17_7: cast zx s_17_6 -> i
        let s_17_7: i128 = (i128::try_from(s_17_6).unwrap());
        // D s_17_8: cast zx s_17_5 -> bv
        let s_17_8: Bits = Bits::new(s_17_5 as u128, 32u16);
        // D s_17_9: call X_set(s_17_7, s_17_0, s_17_8)
        let s_17_9: () = X_set(state, tracer, s_17_7, s_17_0, s_17_8);
        // N s_17_10: jump b18
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
        // D s_20_0: read-var acqrel:u8
        let s_20_0: bool = fn_state.acqrel;
        // N s_20_1: branch s_20_0 b23 b21
        if s_20_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var ar <= s_21_0
        fn_state.ar = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var memop:u32
        let s_22_0: u32 = fn_state.memop;
        // C s_22_1: const #0u : u32
        let s_22_1: u32 = 0;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: read-var ar:u8
        let s_22_3: bool = fn_state.ar;
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: const #0u : u8
        let s_22_5: bool = false;
        // D s_22_6: call SPESampleExtendedLoadStore(s_22_3, s_22_4, s_22_5, s_22_2)
        let s_22_6: () = SPESampleExtendedLoadStore(
            state,
            tracer,
            s_22_3,
            s_22_4,
            s_22_5,
            s_22_2,
        );
        // N s_22_7: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var ar <= s_23_0
        fn_state.ar = s_23_0;
        // N s_23_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var address:u64
        let s_24_0: u64 = fn_state.address;
        // D s_24_1: call SPESetDataVirtualAddress(s_24_0)
        let s_24_1: () = SPESetDataVirtualAddress(state, tracer, s_24_0);
        // N s_24_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var dbytes:i64
        let s_25_0: i64 = fn_state.dbytes;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: read-var address:u64
        let s_25_2: u64 = fn_state.address;
        // D s_25_3: read-var accdesc:struct
        let s_25_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_25_4: read-var data:bv
        let s_25_4: Bits = fn_state.data;
        // D s_25_5: call Mem_set(s_25_2, s_25_1, s_25_3, s_25_4)
        let s_25_5: () = Mem_set(state, tracer, s_25_2, s_25_1, s_25_3, s_25_4);
        // N s_25_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var status <= s_26_0
        fn_state.status = s_26_0;
        // N s_26_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#159406 <= s_27_0
        fn_state.gs_159406 = s_27_0;
        // N s_27_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#159404 <= s_28_0
        fn_state.gs_159404 = s_28_0;
        // N s_28_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#159402 <= s_29_0
        fn_state.gs_159402 = s_29_0;
        // N s_29_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #2s : i
        let s_30_0: i128 = 2;
        // D s_30_1: read-var datasizeshadow#1629:i64
        let s_30_1: i64 = fn_state.datasizeshadow_1629;
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // D s_30_3: div s_30_2 s_30_0
        let s_30_3: i128 = ((s_30_2) / (s_30_0));
        // D s_30_4: cast reint s_30_3 -> i64
        let s_30_4: i64 = (s_30_3 as i64);
        // D s_30_5: cast zx s_30_4 -> i
        let s_30_5: i128 = (i128::try_from(s_30_4).unwrap());
        // D s_30_6: cast reint s_30_5 -> i64
        let s_30_6: i64 = (s_30_5 as i64);
        // D s_30_7: read-var t:i64
        let s_30_7: i64 = fn_state.t;
        // D s_30_8: cast zx s_30_7 -> i
        let s_30_8: i128 = (i128::try_from(s_30_7).unwrap());
        // D s_30_9: call X_read(s_30_8, s_30_6)
        let s_30_9: Bits = X_read(state, tracer, s_30_8, s_30_6);
        // D s_30_10: write-var el1 <= s_30_9
        fn_state.el1 = s_30_9;
        // C s_30_11: const #2s : i
        let s_30_11: i128 = 2;
        // D s_30_12: read-var datasizeshadow#1629:i64
        let s_30_12: i64 = fn_state.datasizeshadow_1629;
        // D s_30_13: cast zx s_30_12 -> i
        let s_30_13: i128 = (i128::try_from(s_30_12).unwrap());
        // D s_30_14: div s_30_13 s_30_11
        let s_30_14: i128 = ((s_30_13) / (s_30_11));
        // D s_30_15: cast reint s_30_14 -> i64
        let s_30_15: i64 = (s_30_14 as i64);
        // D s_30_16: cast zx s_30_15 -> i
        let s_30_16: i128 = (i128::try_from(s_30_15).unwrap());
        // D s_30_17: cast reint s_30_16 -> i64
        let s_30_17: i64 = (s_30_16 as i64);
        // D s_30_18: read-var t2:i64
        let s_30_18: i64 = fn_state.t2;
        // D s_30_19: cast zx s_30_18 -> i
        let s_30_19: i128 = (i128::try_from(s_30_18).unwrap());
        // D s_30_20: call X_read(s_30_19, s_30_17)
        let s_30_20: Bits = X_read(state, tracer, s_30_19, s_30_17);
        // D s_30_21: write-var el2 <= s_30_20
        fn_state.el2 = s_30_20;
        // D s_30_22: read-var accdesc.1:struct
        let s_30_22: u32 = fn_state.accdesc._1;
        // D s_30_23: call BigEndian(s_30_22)
        let s_30_23: bool = BigEndian(state, tracer, s_30_22);
        // N s_30_24: branch s_30_23 b33 b31
        if s_30_23 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var el2:bv
        let s_31_0: Bits = fn_state.el2;
        // D s_31_1: read-var el1:bv
        let s_31_1: Bits = fn_state.el1;
        // D s_31_2: cast reint s_31_0 -> u128
        let s_31_2: u128 = (s_31_0.value() as u128);
        // D s_31_3: size-of s_31_0
        let s_31_3: u16 = s_31_0.length();
        // D s_31_4: cast reint s_31_1 -> u128
        let s_31_4: u128 = (s_31_1.value() as u128);
        // D s_31_5: size-of s_31_1
        let s_31_5: u16 = s_31_1.length();
        // D s_31_6: lsl s_31_2 s_31_5
        let s_31_6: u128 = s_31_2 << s_31_5;
        // D s_31_7: or s_31_6 s_31_4
        let s_31_7: u128 = ((s_31_6) | (s_31_4));
        // D s_31_8: add s_31_3 s_31_5
        let s_31_8: u16 = (s_31_3 + s_31_5);
        // D s_31_9: create-bits s_31_7 s_31_8
        let s_31_9: Bits = Bits::new(s_31_7, s_31_8);
        // D s_31_10: write-var data <= s_31_9
        fn_state.data = s_31_9;
        // N s_31_11: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var el1:bv
        let s_33_0: Bits = fn_state.el1;
        // D s_33_1: read-var el2:bv
        let s_33_1: Bits = fn_state.el2;
        // D s_33_2: cast reint s_33_0 -> u128
        let s_33_2: u128 = (s_33_0.value() as u128);
        // D s_33_3: size-of s_33_0
        let s_33_3: u16 = s_33_0.length();
        // D s_33_4: cast reint s_33_1 -> u128
        let s_33_4: u128 = (s_33_1.value() as u128);
        // D s_33_5: size-of s_33_1
        let s_33_5: u16 = s_33_1.length();
        // D s_33_6: lsl s_33_2 s_33_5
        let s_33_6: u128 = s_33_2 << s_33_5;
        // D s_33_7: or s_33_6 s_33_4
        let s_33_7: u128 = ((s_33_6) | (s_33_4));
        // D s_33_8: add s_33_3 s_33_5
        let s_33_8: u16 = (s_33_3 + s_33_5);
        // D s_33_9: create-bits s_33_7 s_33_8
        let s_33_9: Bits = Bits::new(s_33_7, s_33_8);
        // D s_33_10: write-var data <= s_33_9
        fn_state.data = s_33_9;
        // N s_33_11: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var datasizeshadow#1629:i64
        let s_34_0: i64 = fn_state.datasizeshadow_1629;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: cast reint s_34_1 -> i64
        let s_34_2: i64 = (s_34_1 as i64);
        // D s_34_3: cast zx s_34_2 -> i
        let s_34_3: i128 = (i128::try_from(s_34_2).unwrap());
        // D s_34_4: call __UNKNOWN_bits(s_34_3)
        let s_34_4: Bits = u__UNKNOWN_bits(state, tracer, s_34_3);
        // D s_34_5: write-var data <= s_34_4
        fn_state.data = s_34_4;
        // N s_34_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u32
        let s_35_0: u32 = 0;
        // D s_35_1: read-var memop:u32
        let s_35_1: u32 = fn_state.memop;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // D s_35_3: not s_35_2
        let s_35_3: bool = !s_35_2;
        // N s_35_4: branch s_35_3 b71 b36
        if s_35_3 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var dbytes:i64
        let s_36_0: i64 = fn_state.dbytes;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: read-var address:u64
        let s_36_2: u64 = fn_state.address;
        // D s_36_3: call AArch64_SetExclusiveMonitors(s_36_2, s_36_1)
        let s_36_3: () = AArch64_SetExclusiveMonitors(state, tracer, s_36_2, s_36_1);
        // N s_36_4: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var pair:u8
        let s_37_0: bool = fn_state.pair;
        // N s_37_1: branch s_37_0 b43 b38
        if s_37_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var dbytes:i64
        let s_38_0: i64 = fn_state.dbytes;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: read-var address:u64
        let s_38_2: u64 = fn_state.address;
        // D s_38_3: read-var accdesc:struct
        let s_38_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_38_4: call Mem_read(s_38_2, s_38_1, s_38_3)
        let s_38_4: Bits = Mem_read(state, tracer, s_38_2, s_38_1, s_38_3);
        // D s_38_5: write-var datashadow#1630 <= s_38_4
        fn_state.datashadow_1630 = s_38_4;
        // N s_38_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var datasizeshadow#1629:i64
        let s_39_0: i64 = fn_state.datasizeshadow_1629;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call __id(s_39_1)
        let s_39_2: i128 = u__id(state, tracer, s_39_1);
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #0s : i
        let s_39_4: i128 = 0;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: cmp-ge s_39_5 s_39_4
        let s_39_6: bool = ((s_39_5) >= (s_39_4));
        // N s_39_7: branch s_39_6 b42 b40
        if s_39_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#159418 <= s_40_0
        fn_state.gs_159418 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#159418:u8
        let s_41_0: bool = fn_state.gs_159418;
        // N s_41_1: assert s_41_0
        let s_41_1: () = assert!(s_41_0);
        // D s_41_2: read-var regsizeshadow#1627:i64
        let s_41_2: i64 = fn_state.regsizeshadow_1627;
        // D s_41_3: cast zx s_41_2 -> i
        let s_41_3: i128 = (i128::try_from(s_41_2).unwrap());
        // D s_41_4: cast reint s_41_3 -> i64
        let s_41_4: i64 = (s_41_3 as i64);
        // D s_41_5: read-var regsizeshadow#1627:i64
        let s_41_5: i64 = fn_state.regsizeshadow_1627;
        // D s_41_6: cast zx s_41_5 -> i
        let s_41_6: i128 = (i128::try_from(s_41_5).unwrap());
        // D s_41_7: read-var datashadow#1630:bv
        let s_41_7: Bits = fn_state.datashadow_1630;
        // D s_41_8: bits-cast zx s_41_7 -> bv length s_41_6
        let s_41_8: Bits = s_41_7.zero_extend(s_41_6);
        // D s_41_9: read-var t:i64
        let s_41_9: i64 = fn_state.t;
        // D s_41_10: cast zx s_41_9 -> i
        let s_41_10: i128 = (i128::try_from(s_41_9).unwrap());
        // D s_41_11: call X_set(s_41_10, s_41_4, s_41_8)
        let s_41_11: () = X_set(state, tracer, s_41_10, s_41_4, s_41_8);
        // N s_41_12: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var regsizeshadow#1627:i64
        let s_42_0: i64 = fn_state.regsizeshadow_1627;
        // D s_42_1: cast zx s_42_0 -> i
        let s_42_1: i128 = (i128::try_from(s_42_0).unwrap());
        // D s_42_2: call __id(s_42_1)
        let s_42_2: i128 = u__id(state, tracer, s_42_1);
        // D s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // D s_42_4: read-var datasizeshadow#1629:i64
        let s_42_4: i64 = fn_state.datasizeshadow_1629;
        // D s_42_5: cast zx s_42_4 -> i
        let s_42_5: i128 = (i128::try_from(s_42_4).unwrap());
        // D s_42_6: call __id(s_42_5)
        let s_42_6: i128 = u__id(state, tracer, s_42_5);
        // D s_42_7: cast reint s_42_6 -> i64
        let s_42_7: i64 = (s_42_6 as i64);
        // D s_42_8: cast zx s_42_3 -> i
        let s_42_8: i128 = (i128::try_from(s_42_3).unwrap());
        // D s_42_9: cast zx s_42_7 -> i
        let s_42_9: i128 = (i128::try_from(s_42_7).unwrap());
        // D s_42_10: cmp-ge s_42_8 s_42_9
        let s_42_10: bool = ((s_42_8) >= (s_42_9));
        // D s_42_11: write-var gs#159418 <= s_42_10
        fn_state.gs_159418 = s_42_10;
        // N s_42_12: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var rt_unknown:u8
        let s_43_0: bool = fn_state.rt_unknown;
        // N s_43_1: branch s_43_0 b67 b44
        if s_43_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #32s : i
        let s_44_0: i128 = 32;
        // D s_44_1: read-var elsizeshadow#1628:i64
        let s_44_1: i64 = fn_state.elsizeshadow_1628;
        // D s_44_2: cast zx s_44_1 -> i
        let s_44_2: i128 = (i128::try_from(s_44_1).unwrap());
        // D s_44_3: cmp-eq s_44_2 s_44_0
        let s_44_3: bool = ((s_44_2) == (s_44_0));
        // N s_44_4: branch s_44_3 b51 b45
        if s_44_3 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var address:u64
        let s_45_0: u64 = fn_state.address;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 64u16);
        // D s_45_2: read-var dbytes:i64
        let s_45_2: i64 = fn_state.dbytes;
        // D s_45_3: cast zx s_45_2 -> i
        let s_45_3: i128 = (i128::try_from(s_45_2).unwrap());
        // D s_45_4: call IsAligned__1(s_45_1, s_45_3)
        let s_45_4: bool = IsAligned__1(state, tracer, s_45_1, s_45_3);
        // D s_45_5: not s_45_4
        let s_45_5: bool = !s_45_4;
        // N s_45_6: branch s_45_5 b50 b46
        if s_45_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0s : i
        let s_47_0: i128 = 0;
        // D s_47_1: read-var address:u64
        let s_47_1: u64 = fn_state.address;
        // D s_47_2: cast zx s_47_1 -> bv
        let s_47_2: Bits = Bits::new(s_47_1 as u128, 64u16);
        // C s_47_3: cast cvt s_47_0 -> bv
        let s_47_3: Bits = Bits::new(s_47_0 as u128, 128);
        // D s_47_4: add s_47_2 s_47_3
        let s_47_4: Bits = (s_47_2 + s_47_3);
        // D s_47_5: cast reint s_47_4 -> u64
        let s_47_5: u64 = (s_47_4.value() as u64);
        // C s_47_6: const #8s : i
        let s_47_6: i128 = 8;
        // D s_47_7: read-var accdesc:struct
        let s_47_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_47_8: call Mem_read(s_47_5, s_47_6, s_47_7)
        let s_47_8: Bits = Mem_read(state, tracer, s_47_5, s_47_6, s_47_7);
        // D s_47_9: write-var gs#701903 <= s_47_8
        fn_state.gs_701903 = s_47_8;
        // N s_47_10: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#701903:bv
        let s_48_0: Bits = fn_state.gs_701903;
        // D s_48_1: cast reint s_48_0 -> u64
        let s_48_1: u64 = (s_48_0.value() as u64);
        // D s_48_2: read-var t:i64
        let s_48_2: i64 = fn_state.t;
        // D s_48_3: cast zx s_48_2 -> i
        let s_48_3: i128 = (i128::try_from(s_48_2).unwrap());
        // D s_48_4: cast zx s_48_1 -> bv
        let s_48_4: Bits = Bits::new(s_48_1 as u128, 64u16);
        // C s_48_5: const #64s : i64
        let s_48_5: i64 = 64;
        // D s_48_6: call X_set(s_48_3, s_48_5, s_48_4)
        let s_48_6: () = X_set(state, tracer, s_48_3, s_48_5, s_48_4);
        // C s_48_7: const #8s : i
        let s_48_7: i128 = 8;
        // D s_48_8: read-var address:u64
        let s_48_8: u64 = fn_state.address;
        // D s_48_9: cast zx s_48_8 -> bv
        let s_48_9: Bits = Bits::new(s_48_8 as u128, 64u16);
        // C s_48_10: cast cvt s_48_7 -> bv
        let s_48_10: Bits = Bits::new(s_48_7 as u128, 128);
        // D s_48_11: add s_48_9 s_48_10
        let s_48_11: Bits = (s_48_9 + s_48_10);
        // D s_48_12: cast reint s_48_11 -> u64
        let s_48_12: u64 = (s_48_11.value() as u64);
        // C s_48_13: const #8s : i
        let s_48_13: i128 = 8;
        // D s_48_14: read-var accdesc:struct
        let s_48_14: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_48_15: call Mem_read(s_48_12, s_48_13, s_48_14)
        let s_48_15: Bits = Mem_read(state, tracer, s_48_12, s_48_13, s_48_14);
        // D s_48_16: write-var gs#701909 <= s_48_15
        fn_state.gs_701909 = s_48_15;
        // N s_48_17: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#701909:bv
        let s_49_0: Bits = fn_state.gs_701909;
        // D s_49_1: cast reint s_49_0 -> u64
        let s_49_1: u64 = (s_49_0.value() as u64);
        // D s_49_2: read-var t2:i64
        let s_49_2: i64 = fn_state.t2;
        // D s_49_3: cast zx s_49_2 -> i
        let s_49_3: i128 = (i128::try_from(s_49_2).unwrap());
        // D s_49_4: cast zx s_49_1 -> bv
        let s_49_4: Bits = Bits::new(s_49_1 as u128, 64u16);
        // C s_49_5: const #64s : i64
        let s_49_5: i64 = 64;
        // D s_49_6: call X_set(s_49_3, s_49_5, s_49_4)
        let s_49_6: () = X_set(state, tracer, s_49_3, s_49_5, s_49_4);
        // N s_49_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var accdesc:struct
        let s_50_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_50_1: call AlignmentFault(s_50_0)
        let s_50_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_50_0);
        // D s_50_2: read-var address:u64
        let s_50_2: u64 = fn_state.address;
        // D s_50_3: call AArch64_Abort(s_50_2, s_50_1)
        let s_50_3: () = AArch64_Abort(state, tracer, s_50_2, s_50_1);
        // N s_50_4: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var dbytes:i64
        let s_51_0: i64 = fn_state.dbytes;
        // D s_51_1: cast zx s_51_0 -> i
        let s_51_1: i128 = (i128::try_from(s_51_0).unwrap());
        // D s_51_2: read-var address:u64
        let s_51_2: u64 = fn_state.address;
        // D s_51_3: read-var accdesc:struct
        let s_51_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_51_4: call Mem_read(s_51_2, s_51_1, s_51_3)
        let s_51_4: Bits = Mem_read(state, tracer, s_51_2, s_51_1, s_51_3);
        // D s_51_5: write-var datashadow#1631 <= s_51_4
        fn_state.datashadow_1631 = s_51_4;
        // N s_51_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var accdesc.1:struct
        let s_52_0: u32 = fn_state.accdesc._1;
        // D s_52_1: call BigEndian(s_52_0)
        let s_52_1: bool = BigEndian(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b60 b53
        if s_52_1 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #32s : i64
        let s_53_0: i64 = 32;
        // C s_53_1: const #0s : i
        let s_53_1: i128 = 0;
        // C s_53_2: const #32s : i
        let s_53_2: i128 = 32;
        // D s_53_3: read-var datashadow#1631:bv
        let s_53_3: Bits = fn_state.datashadow_1631;
        // D s_53_4: bit-extract s_53_3 s_53_1 s_53_2
        let s_53_4: Bits = (Bits::new(
            ((s_53_3) >> (s_53_1)).value(),
            u16::try_from(s_53_2).unwrap(),
        ));
        // D s_53_5: cast reint s_53_4 -> u32
        let s_53_5: u32 = (s_53_4.value() as u32);
        // D s_53_6: read-var t:i64
        let s_53_6: i64 = fn_state.t;
        // D s_53_7: cast zx s_53_6 -> i
        let s_53_7: i128 = (i128::try_from(s_53_6).unwrap());
        // D s_53_8: cast zx s_53_5 -> bv
        let s_53_8: Bits = Bits::new(s_53_5 as u128, 32u16);
        // D s_53_9: call X_set(s_53_7, s_53_0, s_53_8)
        let s_53_9: () = X_set(state, tracer, s_53_7, s_53_0, s_53_8);
        // D s_53_10: read-var datasizeshadow#1629:i64
        let s_53_10: i64 = fn_state.datasizeshadow_1629;
        // D s_53_11: cast zx s_53_10 -> i
        let s_53_11: i128 = (i128::try_from(s_53_10).unwrap());
        // D s_53_12: call __id(s_53_11)
        let s_53_12: i128 = u__id(state, tracer, s_53_11);
        // D s_53_13: cast reint s_53_12 -> i64
        let s_53_13: i64 = (s_53_12 as i64);
        // C s_53_14: const #1s : i
        let s_53_14: i128 = 1;
        // D s_53_15: cast zx s_53_13 -> i
        let s_53_15: i128 = (i128::try_from(s_53_13).unwrap());
        // D s_53_16: sub s_53_15 s_53_14
        let s_53_16: i128 = ((s_53_15) - (s_53_14));
        // D s_53_17: cast reint s_53_16 -> i64
        let s_53_17: i64 = (s_53_16 as i64);
        // C s_53_18: const #32s : i
        let s_53_18: i128 = 32;
        // D s_53_19: cast zx s_53_17 -> i
        let s_53_19: i128 = (i128::try_from(s_53_17).unwrap());
        // D s_53_20: cmp-le s_53_18 s_53_19
        let s_53_20: bool = ((s_53_18) <= (s_53_19));
        // N s_53_21: branch s_53_20 b59 b54
        if s_53_20 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#159435 <= s_54_0
        fn_state.gs_159435 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#159435:u8
        let s_55_0: bool = fn_state.gs_159435;
        // N s_55_1: assert s_55_0
        let s_55_1: () = assert!(s_55_0);
        // D s_55_2: read-var datasizeshadow#1629:i64
        let s_55_2: i64 = fn_state.datasizeshadow_1629;
        // D s_55_3: cast zx s_55_2 -> i
        let s_55_3: i128 = (i128::try_from(s_55_2).unwrap());
        // D s_55_4: call __id(s_55_3)
        let s_55_4: i128 = u__id(state, tracer, s_55_3);
        // D s_55_5: cast reint s_55_4 -> i64
        let s_55_5: i64 = (s_55_4 as i64);
        // C s_55_6: const #32s : i
        let s_55_6: i128 = 32;
        // D s_55_7: cast zx s_55_5 -> i
        let s_55_7: i128 = (i128::try_from(s_55_5).unwrap());
        // D s_55_8: sub s_55_7 s_55_6
        let s_55_8: i128 = ((s_55_7) - (s_55_6));
        // D s_55_9: cast reint s_55_8 -> i64
        let s_55_9: i64 = (s_55_8 as i64);
        // C s_55_10: const #32s : i
        let s_55_10: i128 = 32;
        // D s_55_11: cast zx s_55_9 -> i
        let s_55_11: i128 = (i128::try_from(s_55_9).unwrap());
        // D s_55_12: cmp-eq s_55_11 s_55_10
        let s_55_12: bool = ((s_55_11) == (s_55_10));
        // N s_55_13: branch s_55_12 b58 b56
        if s_55_12 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var datasizeshadow#1629:i64
        let s_56_0: i64 = fn_state.datasizeshadow_1629;
        // D s_56_1: cast zx s_56_0 -> i
        let s_56_1: i128 = (i128::try_from(s_56_0).unwrap());
        // D s_56_2: call __id(s_56_1)
        let s_56_2: i128 = u__id(state, tracer, s_56_1);
        // D s_56_3: cast reint s_56_2 -> i64
        let s_56_3: i64 = (s_56_2 as i64);
        // C s_56_4: const #32s : i
        let s_56_4: i128 = 32;
        // D s_56_5: cast zx s_56_3 -> i
        let s_56_5: i128 = (i128::try_from(s_56_3).unwrap());
        // D s_56_6: sub s_56_5 s_56_4
        let s_56_6: i128 = ((s_56_5) - (s_56_4));
        // D s_56_7: cast reint s_56_6 -> i64
        let s_56_7: i64 = (s_56_6 as i64);
        // C s_56_8: const #64s : i
        let s_56_8: i128 = 64;
        // D s_56_9: cast zx s_56_7 -> i
        let s_56_9: i128 = (i128::try_from(s_56_7).unwrap());
        // D s_56_10: cmp-eq s_56_9 s_56_8
        let s_56_10: bool = ((s_56_9) == (s_56_8));
        // D s_56_11: write-var gs#159441 <= s_56_10
        fn_state.gs_159441 = s_56_10;
        // N s_56_12: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#159441:u8
        let s_57_0: bool = fn_state.gs_159441;
        // N s_57_1: assert s_57_0
        let s_57_1: () = assert!(s_57_0);
        // C s_57_2: const #32s : i
        let s_57_2: i128 = 32;
        // D s_57_3: read-var datasizeshadow#1629:i64
        let s_57_3: i64 = fn_state.datasizeshadow_1629;
        // D s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // D s_57_5: sub s_57_4 s_57_2
        let s_57_5: i128 = ((s_57_4) - (s_57_2));
        // D s_57_6: cast reint s_57_5 -> i64
        let s_57_6: i64 = (s_57_5 as i64);
        // D s_57_7: cast zx s_57_6 -> i
        let s_57_7: i128 = (i128::try_from(s_57_6).unwrap());
        // D s_57_8: cast reint s_57_7 -> i64
        let s_57_8: i64 = (s_57_7 as i64);
        // C s_57_9: const #32s : i
        let s_57_9: i128 = 32;
        // C s_57_10: const #32s : i
        let s_57_10: i128 = 32;
        // D s_57_11: read-var datashadow#1631:bv
        let s_57_11: Bits = fn_state.datashadow_1631;
        // D s_57_12: bit-extract s_57_11 s_57_9 s_57_10
        let s_57_12: Bits = (Bits::new(
            ((s_57_11) >> (s_57_9)).value(),
            u16::try_from(s_57_10).unwrap(),
        ));
        // D s_57_13: cast reint s_57_12 -> u32
        let s_57_13: u32 = (s_57_12.value() as u32);
        // D s_57_14: read-var t2:i64
        let s_57_14: i64 = fn_state.t2;
        // D s_57_15: cast zx s_57_14 -> i
        let s_57_15: i128 = (i128::try_from(s_57_14).unwrap());
        // D s_57_16: cast zx s_57_13 -> bv
        let s_57_16: Bits = Bits::new(s_57_13 as u128, 32u16);
        // D s_57_17: call X_set(s_57_15, s_57_8, s_57_16)
        let s_57_17: () = X_set(state, tracer, s_57_15, s_57_8, s_57_16);
        // N s_57_18: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#159441 <= s_58_0
        fn_state.gs_159441 = s_58_0;
        // N s_58_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var datasizeshadow#1629:i64
        let s_59_0: i64 = fn_state.datasizeshadow_1629;
        // D s_59_1: cast zx s_59_0 -> i
        let s_59_1: i128 = (i128::try_from(s_59_0).unwrap());
        // D s_59_2: call __id(s_59_1)
        let s_59_2: i128 = u__id(state, tracer, s_59_1);
        // D s_59_3: cast reint s_59_2 -> i64
        let s_59_3: i64 = (s_59_2 as i64);
        // C s_59_4: const #1s : i
        let s_59_4: i128 = 1;
        // D s_59_5: cast zx s_59_3 -> i
        let s_59_5: i128 = (i128::try_from(s_59_3).unwrap());
        // D s_59_6: sub s_59_5 s_59_4
        let s_59_6: i128 = ((s_59_5) - (s_59_4));
        // D s_59_7: cast reint s_59_6 -> i64
        let s_59_7: i64 = (s_59_6 as i64);
        // D s_59_8: read-var datasizeshadow#1629:i64
        let s_59_8: i64 = fn_state.datasizeshadow_1629;
        // D s_59_9: cast zx s_59_8 -> i
        let s_59_9: i128 = (i128::try_from(s_59_8).unwrap());
        // D s_59_10: call __id(s_59_9)
        let s_59_10: i128 = u__id(state, tracer, s_59_9);
        // D s_59_11: cast reint s_59_10 -> i64
        let s_59_11: i64 = (s_59_10 as i64);
        // D s_59_12: cast zx s_59_7 -> i
        let s_59_12: i128 = (i128::try_from(s_59_7).unwrap());
        // D s_59_13: cast zx s_59_11 -> i
        let s_59_13: i128 = (i128::try_from(s_59_11).unwrap());
        // D s_59_14: cmp-lt s_59_12 s_59_13
        let s_59_14: bool = ((s_59_12) < (s_59_13));
        // D s_59_15: write-var gs#159435 <= s_59_14
        fn_state.gs_159435 = s_59_14;
        // N s_59_16: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var datasizeshadow#1629:i64
        let s_60_0: i64 = fn_state.datasizeshadow_1629;
        // D s_60_1: cast zx s_60_0 -> i
        let s_60_1: i128 = (i128::try_from(s_60_0).unwrap());
        // D s_60_2: call __id(s_60_1)
        let s_60_2: i128 = u__id(state, tracer, s_60_1);
        // D s_60_3: cast reint s_60_2 -> i64
        let s_60_3: i64 = (s_60_2 as i64);
        // C s_60_4: const #1s : i
        let s_60_4: i128 = 1;
        // D s_60_5: cast zx s_60_3 -> i
        let s_60_5: i128 = (i128::try_from(s_60_3).unwrap());
        // D s_60_6: sub s_60_5 s_60_4
        let s_60_6: i128 = ((s_60_5) - (s_60_4));
        // D s_60_7: cast reint s_60_6 -> i64
        let s_60_7: i64 = (s_60_6 as i64);
        // C s_60_8: const #32s : i
        let s_60_8: i128 = 32;
        // D s_60_9: cast zx s_60_7 -> i
        let s_60_9: i128 = (i128::try_from(s_60_7).unwrap());
        // D s_60_10: cmp-le s_60_8 s_60_9
        let s_60_10: bool = ((s_60_8) <= (s_60_9));
        // N s_60_11: branch s_60_10 b66 b61
        if s_60_10 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#159451 <= s_61_0
        fn_state.gs_159451 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#159451:u8
        let s_62_0: bool = fn_state.gs_159451;
        // N s_62_1: assert s_62_0
        let s_62_1: () = assert!(s_62_0);
        // D s_62_2: read-var datasizeshadow#1629:i64
        let s_62_2: i64 = fn_state.datasizeshadow_1629;
        // D s_62_3: cast zx s_62_2 -> i
        let s_62_3: i128 = (i128::try_from(s_62_2).unwrap());
        // D s_62_4: call __id(s_62_3)
        let s_62_4: i128 = u__id(state, tracer, s_62_3);
        // D s_62_5: cast reint s_62_4 -> i64
        let s_62_5: i64 = (s_62_4 as i64);
        // C s_62_6: const #32s : i
        let s_62_6: i128 = 32;
        // D s_62_7: cast zx s_62_5 -> i
        let s_62_7: i128 = (i128::try_from(s_62_5).unwrap());
        // D s_62_8: sub s_62_7 s_62_6
        let s_62_8: i128 = ((s_62_7) - (s_62_6));
        // D s_62_9: cast reint s_62_8 -> i64
        let s_62_9: i64 = (s_62_8 as i64);
        // C s_62_10: const #32s : i
        let s_62_10: i128 = 32;
        // D s_62_11: cast zx s_62_9 -> i
        let s_62_11: i128 = (i128::try_from(s_62_9).unwrap());
        // D s_62_12: cmp-eq s_62_11 s_62_10
        let s_62_12: bool = ((s_62_11) == (s_62_10));
        // N s_62_13: branch s_62_12 b65 b63
        if s_62_12 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var datasizeshadow#1629:i64
        let s_63_0: i64 = fn_state.datasizeshadow_1629;
        // D s_63_1: cast zx s_63_0 -> i
        let s_63_1: i128 = (i128::try_from(s_63_0).unwrap());
        // D s_63_2: call __id(s_63_1)
        let s_63_2: i128 = u__id(state, tracer, s_63_1);
        // D s_63_3: cast reint s_63_2 -> i64
        let s_63_3: i64 = (s_63_2 as i64);
        // C s_63_4: const #32s : i
        let s_63_4: i128 = 32;
        // D s_63_5: cast zx s_63_3 -> i
        let s_63_5: i128 = (i128::try_from(s_63_3).unwrap());
        // D s_63_6: sub s_63_5 s_63_4
        let s_63_6: i128 = ((s_63_5) - (s_63_4));
        // D s_63_7: cast reint s_63_6 -> i64
        let s_63_7: i64 = (s_63_6 as i64);
        // C s_63_8: const #64s : i
        let s_63_8: i128 = 64;
        // D s_63_9: cast zx s_63_7 -> i
        let s_63_9: i128 = (i128::try_from(s_63_7).unwrap());
        // D s_63_10: cmp-eq s_63_9 s_63_8
        let s_63_10: bool = ((s_63_9) == (s_63_8));
        // D s_63_11: write-var gs#159457 <= s_63_10
        fn_state.gs_159457 = s_63_10;
        // N s_63_12: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#159457:u8
        let s_64_0: bool = fn_state.gs_159457;
        // N s_64_1: assert s_64_0
        let s_64_1: () = assert!(s_64_0);
        // C s_64_2: const #32s : i
        let s_64_2: i128 = 32;
        // D s_64_3: read-var datasizeshadow#1629:i64
        let s_64_3: i64 = fn_state.datasizeshadow_1629;
        // D s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // D s_64_5: sub s_64_4 s_64_2
        let s_64_5: i128 = ((s_64_4) - (s_64_2));
        // D s_64_6: cast reint s_64_5 -> i64
        let s_64_6: i64 = (s_64_5 as i64);
        // D s_64_7: cast zx s_64_6 -> i
        let s_64_7: i128 = (i128::try_from(s_64_6).unwrap());
        // D s_64_8: cast reint s_64_7 -> i64
        let s_64_8: i64 = (s_64_7 as i64);
        // C s_64_9: const #32s : i
        let s_64_9: i128 = 32;
        // C s_64_10: const #32s : i
        let s_64_10: i128 = 32;
        // D s_64_11: read-var datashadow#1631:bv
        let s_64_11: Bits = fn_state.datashadow_1631;
        // D s_64_12: bit-extract s_64_11 s_64_9 s_64_10
        let s_64_12: Bits = (Bits::new(
            ((s_64_11) >> (s_64_9)).value(),
            u16::try_from(s_64_10).unwrap(),
        ));
        // D s_64_13: cast reint s_64_12 -> u32
        let s_64_13: u32 = (s_64_12.value() as u32);
        // D s_64_14: read-var t:i64
        let s_64_14: i64 = fn_state.t;
        // D s_64_15: cast zx s_64_14 -> i
        let s_64_15: i128 = (i128::try_from(s_64_14).unwrap());
        // D s_64_16: cast zx s_64_13 -> bv
        let s_64_16: Bits = Bits::new(s_64_13 as u128, 32u16);
        // D s_64_17: call X_set(s_64_15, s_64_8, s_64_16)
        let s_64_17: () = X_set(state, tracer, s_64_15, s_64_8, s_64_16);
        // C s_64_18: const #32s : i64
        let s_64_18: i64 = 32;
        // C s_64_19: const #0s : i
        let s_64_19: i128 = 0;
        // C s_64_20: const #32s : i
        let s_64_20: i128 = 32;
        // D s_64_21: read-var datashadow#1631:bv
        let s_64_21: Bits = fn_state.datashadow_1631;
        // D s_64_22: bit-extract s_64_21 s_64_19 s_64_20
        let s_64_22: Bits = (Bits::new(
            ((s_64_21) >> (s_64_19)).value(),
            u16::try_from(s_64_20).unwrap(),
        ));
        // D s_64_23: cast reint s_64_22 -> u32
        let s_64_23: u32 = (s_64_22.value() as u32);
        // D s_64_24: read-var t2:i64
        let s_64_24: i64 = fn_state.t2;
        // D s_64_25: cast zx s_64_24 -> i
        let s_64_25: i128 = (i128::try_from(s_64_24).unwrap());
        // D s_64_26: cast zx s_64_23 -> bv
        let s_64_26: Bits = Bits::new(s_64_23 as u128, 32u16);
        // D s_64_27: call X_set(s_64_25, s_64_18, s_64_26)
        let s_64_27: () = X_set(state, tracer, s_64_25, s_64_18, s_64_26);
        // N s_64_28: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#159457 <= s_65_0
        fn_state.gs_159457 = s_65_0;
        // N s_65_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var datasizeshadow#1629:i64
        let s_66_0: i64 = fn_state.datasizeshadow_1629;
        // D s_66_1: cast zx s_66_0 -> i
        let s_66_1: i128 = (i128::try_from(s_66_0).unwrap());
        // D s_66_2: call __id(s_66_1)
        let s_66_2: i128 = u__id(state, tracer, s_66_1);
        // D s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: const #1s : i
        let s_66_4: i128 = 1;
        // D s_66_5: cast zx s_66_3 -> i
        let s_66_5: i128 = (i128::try_from(s_66_3).unwrap());
        // D s_66_6: sub s_66_5 s_66_4
        let s_66_6: i128 = ((s_66_5) - (s_66_4));
        // D s_66_7: cast reint s_66_6 -> i64
        let s_66_7: i64 = (s_66_6 as i64);
        // D s_66_8: read-var datasizeshadow#1629:i64
        let s_66_8: i64 = fn_state.datasizeshadow_1629;
        // D s_66_9: cast zx s_66_8 -> i
        let s_66_9: i128 = (i128::try_from(s_66_8).unwrap());
        // D s_66_10: call __id(s_66_9)
        let s_66_10: i128 = u__id(state, tracer, s_66_9);
        // D s_66_11: cast reint s_66_10 -> i64
        let s_66_11: i64 = (s_66_10 as i64);
        // D s_66_12: cast zx s_66_7 -> i
        let s_66_12: i128 = (i128::try_from(s_66_7).unwrap());
        // D s_66_13: cast zx s_66_11 -> i
        let s_66_13: i128 = (i128::try_from(s_66_11).unwrap());
        // D s_66_14: cmp-lt s_66_12 s_66_13
        let s_66_14: bool = ((s_66_12) < (s_66_13));
        // D s_66_15: write-var gs#159451 <= s_66_14
        fn_state.gs_159451 = s_66_14;
        // N s_66_16: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var datasizeshadow#1629:i64
        let s_67_0: i64 = fn_state.datasizeshadow_1629;
        // D s_67_1: cast zx s_67_0 -> i
        let s_67_1: i128 = (i128::try_from(s_67_0).unwrap());
        // D s_67_2: call __id(s_67_1)
        let s_67_2: i128 = u__id(state, tracer, s_67_1);
        // D s_67_3: cast reint s_67_2 -> i64
        let s_67_3: i64 = (s_67_2 as i64);
        // C s_67_4: const #32s : i
        let s_67_4: i128 = 32;
        // D s_67_5: cast zx s_67_3 -> i
        let s_67_5: i128 = (i128::try_from(s_67_3).unwrap());
        // D s_67_6: cmp-eq s_67_5 s_67_4
        let s_67_6: bool = ((s_67_5) == (s_67_4));
        // N s_67_7: branch s_67_6 b70 b68
        if s_67_6 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var datasizeshadow#1629:i64
        let s_68_0: i64 = fn_state.datasizeshadow_1629;
        // D s_68_1: cast zx s_68_0 -> i
        let s_68_1: i128 = (i128::try_from(s_68_0).unwrap());
        // D s_68_2: call __id(s_68_1)
        let s_68_2: i128 = u__id(state, tracer, s_68_1);
        // D s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: const #64s : i
        let s_68_4: i128 = 64;
        // D s_68_5: cast zx s_68_3 -> i
        let s_68_5: i128 = (i128::try_from(s_68_3).unwrap());
        // D s_68_6: cmp-eq s_68_5 s_68_4
        let s_68_6: bool = ((s_68_5) == (s_68_4));
        // D s_68_7: write-var gs#159469 <= s_68_6
        fn_state.gs_159469 = s_68_6;
        // N s_68_8: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#159469:u8
        let s_69_0: bool = fn_state.gs_159469;
        // N s_69_1: assert s_69_0
        let s_69_1: () = assert!(s_69_0);
        // D s_69_2: read-var datasizeshadow#1629:i64
        let s_69_2: i64 = fn_state.datasizeshadow_1629;
        // D s_69_3: cast zx s_69_2 -> i
        let s_69_3: i128 = (i128::try_from(s_69_2).unwrap());
        // D s_69_4: cast reint s_69_3 -> i64
        let s_69_4: i64 = (s_69_3 as i64);
        // D s_69_5: read-var datasizeshadow#1629:i64
        let s_69_5: i64 = fn_state.datasizeshadow_1629;
        // D s_69_6: cast zx s_69_5 -> i
        let s_69_6: i128 = (i128::try_from(s_69_5).unwrap());
        // D s_69_7: cast reint s_69_6 -> i64
        let s_69_7: i64 = (s_69_6 as i64);
        // D s_69_8: cast zx s_69_7 -> i
        let s_69_8: i128 = (i128::try_from(s_69_7).unwrap());
        // D s_69_9: call __UNKNOWN_bits(s_69_8)
        let s_69_9: Bits = u__UNKNOWN_bits(state, tracer, s_69_8);
        // D s_69_10: read-var t:i64
        let s_69_10: i64 = fn_state.t;
        // D s_69_11: cast zx s_69_10 -> i
        let s_69_11: i128 = (i128::try_from(s_69_10).unwrap());
        // D s_69_12: call X_set(s_69_11, s_69_4, s_69_9)
        let s_69_12: () = X_set(state, tracer, s_69_11, s_69_4, s_69_9);
        // N s_69_13: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#159469 <= s_70_0
        fn_state.gs_159469 = s_70_0;
        // N s_70_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #64s : i64
        let s_72_0: i64 = 64;
        // C s_72_1: cast zx s_72_0 -> i
        let s_72_1: i128 = (i128::try_from(s_72_0).unwrap());
        // S s_72_2: call __UNKNOWN_bits(s_72_1)
        let s_72_2: Bits = u__UNKNOWN_bits(state, tracer, s_72_1);
        // S s_72_3: cast reint s_72_2 -> u64
        let s_72_3: u64 = (s_72_2.value() as u64);
        // D s_72_4: write-var address <= s_72_3
        fn_state.address = s_72_3;
        // N s_72_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call CheckSPAlignment(s_73_0)
        let s_73_1: () = CheckSPAlignment(state, tracer, s_73_0);
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call SP_read(s_74_0)
        let s_74_1: u64 = SP_read(state, tracer, s_74_0);
        // D s_74_2: write-var address <= s_74_1
        fn_state.address = s_74_1;
        // N s_74_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
