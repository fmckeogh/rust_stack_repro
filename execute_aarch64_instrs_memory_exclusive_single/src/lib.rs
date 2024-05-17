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
pub fn execute_aarch64_instrs_memory_exclusive_single<T: Tracer>(
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
        gs_159772: bool,
        address: u64,
        datashadow_1635: Bits,
        datasizeshadow_1634: i64,
        gs_159702: bool,
        el2: Bits,
        ar: bool,
        gs_159749: bool,
        gs_159784: bool,
        ga_259626: bool,
        datashadow_1636: Bits,
        dbytes: i64,
        gs_159755: bool,
        regsizeshadow_1632: i64,
        el1: Bits,
        gs_159704: bool,
        gs_159716: bool,
        gs_159766: bool,
        gs_702226: Bits,
        gs_702220: Bits,
        gs_159706: bool,
        gs_159718: bool,
        data: Bits,
        gs_159717: bool,
        status: bool,
        accdesc: ProductType9878976b5bcce9c9,
        gs_159730: bool,
        elsizeshadow_1633: i64,
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
        // D s_0_3: write-var regsizeshadow#1632 <= s_0_2
        fn_state.regsizeshadow_1632 = s_0_2;
        // D s_0_4: read-var elsize:i64
        let s_0_4: i64 = fn_state.elsize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var elsizeshadow#1633 <= s_0_6
        fn_state.elsizeshadow_1633 = s_0_6;
        // D s_0_8: read-var datasize:i64
        let s_0_8: i64 = fn_state.datasize;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var datasizeshadow#1634 <= s_0_10
        fn_state.datasizeshadow_1634 = s_0_10;
        // C s_0_12: const #8s : i
        let s_0_12: i128 = 8;
        // D s_0_13: read-var datasizeshadow#1634:i64
        let s_0_13: i64 = fn_state.datasizeshadow_1634;
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
        // N s_0_27: branch s_0_26 b82 b1
        if s_0_26 {
            return block_82(state, tracer, fn_state);
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
        // N s_1_1: branch s_1_0 b81 b2
        if s_1_0 {
            return block_81(state, tracer, fn_state);
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
        // N s_3_4: branch s_3_3 b44 b4
        if s_3_3 {
            return block_44(state, tracer, fn_state);
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
        // N s_4_1: branch s_4_0 b43 b5
        if s_4_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_6_0: read-var datasizeshadow#1634:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1634;
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
        // D s_7_0: read-var datasizeshadow#1634:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1634;
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
        // D s_7_7: write-var gs#159702 <= s_7_6
        fn_state.gs_159702 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#159702:u8
        let s_8_0: bool = fn_state.gs_159702;
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
        // D s_9_0: read-var datasizeshadow#1634:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1634;
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
        // D s_9_7: write-var gs#159704 <= s_9_6
        fn_state.gs_159704 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#159704:u8
        let s_10_0: bool = fn_state.gs_159704;
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
        // D s_11_0: read-var datasizeshadow#1634:i64
        let s_11_0: i64 = fn_state.datasizeshadow_1634;
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
        // D s_11_7: write-var gs#159706 <= s_11_6
        fn_state.gs_159706 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#159706:u8
        let s_12_0: bool = fn_state.gs_159706;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var datasizeshadow#1634:i64
        let s_12_2: i64 = fn_state.datasizeshadow_1634;
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
        // D s_13_6: write-var ga#259626 <= s_13_5
        fn_state.ga_259626 = s_13_5;
        // N s_13_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#259626:u8
        let s_14_0: bool = fn_state.ga_259626;
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
        // D s_27_1: write-var gs#159706 <= s_27_0
        fn_state.gs_159706 = s_27_0;
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
        // D s_28_1: write-var gs#159704 <= s_28_0
        fn_state.gs_159704 = s_28_0;
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
        // D s_29_1: write-var gs#159702 <= s_29_0
        fn_state.gs_159702 = s_29_0;
        // N s_29_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var datasizeshadow#1634:i64
        let s_30_0: i64 = fn_state.datasizeshadow_1634;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #2s : i
        let s_30_4: i128 = 2;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: div s_30_5 s_30_4
        let s_30_6: i128 = ((s_30_5) / (s_30_4));
        // D s_30_7: cast reint s_30_6 -> i64
        let s_30_7: i64 = (s_30_6 as i64);
        // C s_30_8: const #8s : i
        let s_30_8: i128 = 8;
        // D s_30_9: cast zx s_30_7 -> i
        let s_30_9: i128 = (i128::try_from(s_30_7).unwrap());
        // D s_30_10: cmp-eq s_30_9 s_30_8
        let s_30_10: bool = ((s_30_9) == (s_30_8));
        // N s_30_11: branch s_30_10 b42 b31
        if s_30_10 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var datasizeshadow#1634:i64
        let s_31_0: i64 = fn_state.datasizeshadow_1634;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #2s : i
        let s_31_4: i128 = 2;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: div s_31_5 s_31_4
        let s_31_6: i128 = ((s_31_5) / (s_31_4));
        // D s_31_7: cast reint s_31_6 -> i64
        let s_31_7: i64 = (s_31_6 as i64);
        // C s_31_8: const #16s : i
        let s_31_8: i128 = 16;
        // D s_31_9: cast zx s_31_7 -> i
        let s_31_9: i128 = (i128::try_from(s_31_7).unwrap());
        // D s_31_10: cmp-eq s_31_9 s_31_8
        let s_31_10: bool = ((s_31_9) == (s_31_8));
        // N s_31_11: branch s_31_10 b41 b32
        if s_31_10 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var datasizeshadow#1634:i64
        let s_32_0: i64 = fn_state.datasizeshadow_1634;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call __id(s_32_1)
        let s_32_2: i128 = u__id(state, tracer, s_32_1);
        // D s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: const #2s : i
        let s_32_4: i128 = 2;
        // D s_32_5: cast zx s_32_3 -> i
        let s_32_5: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_6: div s_32_5 s_32_4
        let s_32_6: i128 = ((s_32_5) / (s_32_4));
        // D s_32_7: cast reint s_32_6 -> i64
        let s_32_7: i64 = (s_32_6 as i64);
        // C s_32_8: const #32s : i
        let s_32_8: i128 = 32;
        // D s_32_9: cast zx s_32_7 -> i
        let s_32_9: i128 = (i128::try_from(s_32_7).unwrap());
        // D s_32_10: cmp-eq s_32_9 s_32_8
        let s_32_10: bool = ((s_32_9) == (s_32_8));
        // N s_32_11: branch s_32_10 b40 b33
        if s_32_10 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var datasizeshadow#1634:i64
        let s_33_0: i64 = fn_state.datasizeshadow_1634;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #2s : i
        let s_33_4: i128 = 2;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: div s_33_5 s_33_4
        let s_33_6: i128 = ((s_33_5) / (s_33_4));
        // D s_33_7: cast reint s_33_6 -> i64
        let s_33_7: i64 = (s_33_6 as i64);
        // C s_33_8: const #64s : i
        let s_33_8: i128 = 64;
        // D s_33_9: cast zx s_33_7 -> i
        let s_33_9: i128 = (i128::try_from(s_33_7).unwrap());
        // D s_33_10: cmp-eq s_33_9 s_33_8
        let s_33_10: bool = ((s_33_9) == (s_33_8));
        // D s_33_11: write-var gs#159716 <= s_33_10
        fn_state.gs_159716 = s_33_10;
        // N s_33_12: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#159716:u8
        let s_34_0: bool = fn_state.gs_159716;
        // D s_34_1: write-var gs#159717 <= s_34_0
        fn_state.gs_159717 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#159717:u8
        let s_35_0: bool = fn_state.gs_159717;
        // D s_35_1: write-var gs#159718 <= s_35_0
        fn_state.gs_159718 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#159718:u8
        let s_36_0: bool = fn_state.gs_159718;
        // N s_36_1: assert s_36_0
        let s_36_1: () = assert!(s_36_0);
        // C s_36_2: const #2s : i
        let s_36_2: i128 = 2;
        // D s_36_3: read-var datasizeshadow#1634:i64
        let s_36_3: i64 = fn_state.datasizeshadow_1634;
        // D s_36_4: cast zx s_36_3 -> i
        let s_36_4: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_5: div s_36_4 s_36_2
        let s_36_5: i128 = ((s_36_4) / (s_36_2));
        // D s_36_6: cast reint s_36_5 -> i64
        let s_36_6: i64 = (s_36_5 as i64);
        // D s_36_7: cast zx s_36_6 -> i
        let s_36_7: i128 = (i128::try_from(s_36_6).unwrap());
        // D s_36_8: cast reint s_36_7 -> i64
        let s_36_8: i64 = (s_36_7 as i64);
        // D s_36_9: read-var t:i64
        let s_36_9: i64 = fn_state.t;
        // D s_36_10: cast zx s_36_9 -> i
        let s_36_10: i128 = (i128::try_from(s_36_9).unwrap());
        // D s_36_11: call X_read(s_36_10, s_36_8)
        let s_36_11: Bits = X_read(state, tracer, s_36_10, s_36_8);
        // D s_36_12: write-var el1 <= s_36_11
        fn_state.el1 = s_36_11;
        // C s_36_13: const #2s : i
        let s_36_13: i128 = 2;
        // D s_36_14: read-var datasizeshadow#1634:i64
        let s_36_14: i64 = fn_state.datasizeshadow_1634;
        // D s_36_15: cast zx s_36_14 -> i
        let s_36_15: i128 = (i128::try_from(s_36_14).unwrap());
        // D s_36_16: div s_36_15 s_36_13
        let s_36_16: i128 = ((s_36_15) / (s_36_13));
        // D s_36_17: cast reint s_36_16 -> i64
        let s_36_17: i64 = (s_36_16 as i64);
        // D s_36_18: cast zx s_36_17 -> i
        let s_36_18: i128 = (i128::try_from(s_36_17).unwrap());
        // D s_36_19: cast reint s_36_18 -> i64
        let s_36_19: i64 = (s_36_18 as i64);
        // D s_36_20: read-var t2:i64
        let s_36_20: i64 = fn_state.t2;
        // D s_36_21: cast zx s_36_20 -> i
        let s_36_21: i128 = (i128::try_from(s_36_20).unwrap());
        // D s_36_22: call X_read(s_36_21, s_36_19)
        let s_36_22: Bits = X_read(state, tracer, s_36_21, s_36_19);
        // D s_36_23: write-var el2 <= s_36_22
        fn_state.el2 = s_36_22;
        // D s_36_24: read-var accdesc.1:struct
        let s_36_24: u32 = fn_state.accdesc._1;
        // D s_36_25: call BigEndian(s_36_24)
        let s_36_25: bool = BigEndian(state, tracer, s_36_24);
        // N s_36_26: branch s_36_25 b39 b37
        if s_36_25 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var el2:bv
        let s_37_0: Bits = fn_state.el2;
        // D s_37_1: read-var el1:bv
        let s_37_1: Bits = fn_state.el1;
        // D s_37_2: cast reint s_37_0 -> u128
        let s_37_2: u128 = (s_37_0.value() as u128);
        // D s_37_3: size-of s_37_0
        let s_37_3: u16 = s_37_0.length();
        // D s_37_4: cast reint s_37_1 -> u128
        let s_37_4: u128 = (s_37_1.value() as u128);
        // D s_37_5: size-of s_37_1
        let s_37_5: u16 = s_37_1.length();
        // D s_37_6: lsl s_37_2 s_37_5
        let s_37_6: u128 = s_37_2 << s_37_5;
        // D s_37_7: or s_37_6 s_37_4
        let s_37_7: u128 = ((s_37_6) | (s_37_4));
        // D s_37_8: add s_37_3 s_37_5
        let s_37_8: u16 = (s_37_3 + s_37_5);
        // D s_37_9: create-bits s_37_7 s_37_8
        let s_37_9: Bits = Bits::new(s_37_7, s_37_8);
        // D s_37_10: write-var data <= s_37_9
        fn_state.data = s_37_9;
        // N s_37_11: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var el1:bv
        let s_39_0: Bits = fn_state.el1;
        // D s_39_1: read-var el2:bv
        let s_39_1: Bits = fn_state.el2;
        // D s_39_2: cast reint s_39_0 -> u128
        let s_39_2: u128 = (s_39_0.value() as u128);
        // D s_39_3: size-of s_39_0
        let s_39_3: u16 = s_39_0.length();
        // D s_39_4: cast reint s_39_1 -> u128
        let s_39_4: u128 = (s_39_1.value() as u128);
        // D s_39_5: size-of s_39_1
        let s_39_5: u16 = s_39_1.length();
        // D s_39_6: lsl s_39_2 s_39_5
        let s_39_6: u128 = s_39_2 << s_39_5;
        // D s_39_7: or s_39_6 s_39_4
        let s_39_7: u128 = ((s_39_6) | (s_39_4));
        // D s_39_8: add s_39_3 s_39_5
        let s_39_8: u16 = (s_39_3 + s_39_5);
        // D s_39_9: create-bits s_39_7 s_39_8
        let s_39_9: Bits = Bits::new(s_39_7, s_39_8);
        // D s_39_10: write-var data <= s_39_9
        fn_state.data = s_39_9;
        // N s_39_11: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#159716 <= s_40_0
        fn_state.gs_159716 = s_40_0;
        // N s_40_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#159717 <= s_41_0
        fn_state.gs_159717 = s_41_0;
        // N s_41_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#159718 <= s_42_0
        fn_state.gs_159718 = s_42_0;
        // N s_42_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var datasizeshadow#1634:i64
        let s_43_0: i64 = fn_state.datasizeshadow_1634;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: cast reint s_43_1 -> i64
        let s_43_2: i64 = (s_43_1 as i64);
        // D s_43_3: cast zx s_43_2 -> i
        let s_43_3: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_4: call __UNKNOWN_bits(s_43_3)
        let s_43_4: Bits = u__UNKNOWN_bits(state, tracer, s_43_3);
        // D s_43_5: write-var data <= s_43_4
        fn_state.data = s_43_4;
        // N s_43_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u32
        let s_44_0: u32 = 0;
        // D s_44_1: read-var memop:u32
        let s_44_1: u32 = fn_state.memop;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // N s_44_4: branch s_44_3 b80 b45
        if s_44_3 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var dbytes:i64
        let s_45_0: i64 = fn_state.dbytes;
        // D s_45_1: cast zx s_45_0 -> i
        let s_45_1: i128 = (i128::try_from(s_45_0).unwrap());
        // D s_45_2: read-var address:u64
        let s_45_2: u64 = fn_state.address;
        // D s_45_3: call AArch64_SetExclusiveMonitors(s_45_2, s_45_1)
        let s_45_3: () = AArch64_SetExclusiveMonitors(state, tracer, s_45_2, s_45_1);
        // N s_45_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var pair:u8
        let s_46_0: bool = fn_state.pair;
        // N s_46_1: branch s_46_0 b52 b47
        if s_46_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var dbytes:i64
        let s_47_0: i64 = fn_state.dbytes;
        // D s_47_1: cast zx s_47_0 -> i
        let s_47_1: i128 = (i128::try_from(s_47_0).unwrap());
        // D s_47_2: read-var address:u64
        let s_47_2: u64 = fn_state.address;
        // D s_47_3: read-var accdesc:struct
        let s_47_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_47_4: call Mem_read(s_47_2, s_47_1, s_47_3)
        let s_47_4: Bits = Mem_read(state, tracer, s_47_2, s_47_1, s_47_3);
        // D s_47_5: write-var datashadow#1635 <= s_47_4
        fn_state.datashadow_1635 = s_47_4;
        // N s_47_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var datasizeshadow#1634:i64
        let s_48_0: i64 = fn_state.datasizeshadow_1634;
        // D s_48_1: cast zx s_48_0 -> i
        let s_48_1: i128 = (i128::try_from(s_48_0).unwrap());
        // D s_48_2: call __id(s_48_1)
        let s_48_2: i128 = u__id(state, tracer, s_48_1);
        // D s_48_3: cast reint s_48_2 -> i64
        let s_48_3: i64 = (s_48_2 as i64);
        // C s_48_4: const #0s : i
        let s_48_4: i128 = 0;
        // D s_48_5: cast zx s_48_3 -> i
        let s_48_5: i128 = (i128::try_from(s_48_3).unwrap());
        // D s_48_6: cmp-ge s_48_5 s_48_4
        let s_48_6: bool = ((s_48_5) >= (s_48_4));
        // N s_48_7: branch s_48_6 b51 b49
        if s_48_6 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#159730 <= s_49_0
        fn_state.gs_159730 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#159730:u8
        let s_50_0: bool = fn_state.gs_159730;
        // N s_50_1: assert s_50_0
        let s_50_1: () = assert!(s_50_0);
        // D s_50_2: read-var regsizeshadow#1632:i64
        let s_50_2: i64 = fn_state.regsizeshadow_1632;
        // D s_50_3: cast zx s_50_2 -> i
        let s_50_3: i128 = (i128::try_from(s_50_2).unwrap());
        // D s_50_4: cast reint s_50_3 -> i64
        let s_50_4: i64 = (s_50_3 as i64);
        // D s_50_5: read-var regsizeshadow#1632:i64
        let s_50_5: i64 = fn_state.regsizeshadow_1632;
        // D s_50_6: cast zx s_50_5 -> i
        let s_50_6: i128 = (i128::try_from(s_50_5).unwrap());
        // D s_50_7: read-var datashadow#1635:bv
        let s_50_7: Bits = fn_state.datashadow_1635;
        // D s_50_8: bits-cast zx s_50_7 -> bv length s_50_6
        let s_50_8: Bits = s_50_7.zero_extend(s_50_6);
        // D s_50_9: read-var t:i64
        let s_50_9: i64 = fn_state.t;
        // D s_50_10: cast zx s_50_9 -> i
        let s_50_10: i128 = (i128::try_from(s_50_9).unwrap());
        // D s_50_11: call X_set(s_50_10, s_50_4, s_50_8)
        let s_50_11: () = X_set(state, tracer, s_50_10, s_50_4, s_50_8);
        // N s_50_12: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var regsizeshadow#1632:i64
        let s_51_0: i64 = fn_state.regsizeshadow_1632;
        // D s_51_1: cast zx s_51_0 -> i
        let s_51_1: i128 = (i128::try_from(s_51_0).unwrap());
        // D s_51_2: call __id(s_51_1)
        let s_51_2: i128 = u__id(state, tracer, s_51_1);
        // D s_51_3: cast reint s_51_2 -> i64
        let s_51_3: i64 = (s_51_2 as i64);
        // D s_51_4: read-var datasizeshadow#1634:i64
        let s_51_4: i64 = fn_state.datasizeshadow_1634;
        // D s_51_5: cast zx s_51_4 -> i
        let s_51_5: i128 = (i128::try_from(s_51_4).unwrap());
        // D s_51_6: call __id(s_51_5)
        let s_51_6: i128 = u__id(state, tracer, s_51_5);
        // D s_51_7: cast reint s_51_6 -> i64
        let s_51_7: i64 = (s_51_6 as i64);
        // D s_51_8: cast zx s_51_3 -> i
        let s_51_8: i128 = (i128::try_from(s_51_3).unwrap());
        // D s_51_9: cast zx s_51_7 -> i
        let s_51_9: i128 = (i128::try_from(s_51_7).unwrap());
        // D s_51_10: cmp-ge s_51_8 s_51_9
        let s_51_10: bool = ((s_51_8) >= (s_51_9));
        // D s_51_11: write-var gs#159730 <= s_51_10
        fn_state.gs_159730 = s_51_10;
        // N s_51_12: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var rt_unknown:u8
        let s_52_0: bool = fn_state.rt_unknown;
        // N s_52_1: branch s_52_0 b76 b53
        if s_52_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #32s : i
        let s_53_0: i128 = 32;
        // D s_53_1: read-var elsizeshadow#1633:i64
        let s_53_1: i64 = fn_state.elsizeshadow_1633;
        // D s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (i128::try_from(s_53_1).unwrap());
        // D s_53_3: cmp-eq s_53_2 s_53_0
        let s_53_3: bool = ((s_53_2) == (s_53_0));
        // N s_53_4: branch s_53_3 b60 b54
        if s_53_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var address:u64
        let s_54_0: u64 = fn_state.address;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 64u16);
        // D s_54_2: read-var dbytes:i64
        let s_54_2: i64 = fn_state.dbytes;
        // D s_54_3: cast zx s_54_2 -> i
        let s_54_3: i128 = (i128::try_from(s_54_2).unwrap());
        // D s_54_4: call IsAligned__1(s_54_1, s_54_3)
        let s_54_4: bool = IsAligned__1(state, tracer, s_54_1, s_54_3);
        // D s_54_5: not s_54_4
        let s_54_5: bool = !s_54_4;
        // N s_54_6: branch s_54_5 b59 b55
        if s_54_5 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0s : i
        let s_56_0: i128 = 0;
        // D s_56_1: read-var address:u64
        let s_56_1: u64 = fn_state.address;
        // D s_56_2: cast zx s_56_1 -> bv
        let s_56_2: Bits = Bits::new(s_56_1 as u128, 64u16);
        // C s_56_3: cast cvt s_56_0 -> bv
        let s_56_3: Bits = Bits::new(s_56_0 as u128, 128);
        // D s_56_4: add s_56_2 s_56_3
        let s_56_4: Bits = (s_56_2 + s_56_3);
        // D s_56_5: cast reint s_56_4 -> u64
        let s_56_5: u64 = (s_56_4.value() as u64);
        // C s_56_6: const #8s : i
        let s_56_6: i128 = 8;
        // D s_56_7: read-var accdesc:struct
        let s_56_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_56_8: call Mem_read(s_56_5, s_56_6, s_56_7)
        let s_56_8: Bits = Mem_read(state, tracer, s_56_5, s_56_6, s_56_7);
        // D s_56_9: write-var gs#702220 <= s_56_8
        fn_state.gs_702220 = s_56_8;
        // N s_56_10: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#702220:bv
        let s_57_0: Bits = fn_state.gs_702220;
        // D s_57_1: cast reint s_57_0 -> u64
        let s_57_1: u64 = (s_57_0.value() as u64);
        // D s_57_2: read-var t:i64
        let s_57_2: i64 = fn_state.t;
        // D s_57_3: cast zx s_57_2 -> i
        let s_57_3: i128 = (i128::try_from(s_57_2).unwrap());
        // D s_57_4: cast zx s_57_1 -> bv
        let s_57_4: Bits = Bits::new(s_57_1 as u128, 64u16);
        // C s_57_5: const #64s : i64
        let s_57_5: i64 = 64;
        // D s_57_6: call X_set(s_57_3, s_57_5, s_57_4)
        let s_57_6: () = X_set(state, tracer, s_57_3, s_57_5, s_57_4);
        // C s_57_7: const #8s : i
        let s_57_7: i128 = 8;
        // D s_57_8: read-var address:u64
        let s_57_8: u64 = fn_state.address;
        // D s_57_9: cast zx s_57_8 -> bv
        let s_57_9: Bits = Bits::new(s_57_8 as u128, 64u16);
        // C s_57_10: cast cvt s_57_7 -> bv
        let s_57_10: Bits = Bits::new(s_57_7 as u128, 128);
        // D s_57_11: add s_57_9 s_57_10
        let s_57_11: Bits = (s_57_9 + s_57_10);
        // D s_57_12: cast reint s_57_11 -> u64
        let s_57_12: u64 = (s_57_11.value() as u64);
        // C s_57_13: const #8s : i
        let s_57_13: i128 = 8;
        // D s_57_14: read-var accdesc:struct
        let s_57_14: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_57_15: call Mem_read(s_57_12, s_57_13, s_57_14)
        let s_57_15: Bits = Mem_read(state, tracer, s_57_12, s_57_13, s_57_14);
        // D s_57_16: write-var gs#702226 <= s_57_15
        fn_state.gs_702226 = s_57_15;
        // N s_57_17: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#702226:bv
        let s_58_0: Bits = fn_state.gs_702226;
        // D s_58_1: cast reint s_58_0 -> u64
        let s_58_1: u64 = (s_58_0.value() as u64);
        // D s_58_2: read-var t2:i64
        let s_58_2: i64 = fn_state.t2;
        // D s_58_3: cast zx s_58_2 -> i
        let s_58_3: i128 = (i128::try_from(s_58_2).unwrap());
        // D s_58_4: cast zx s_58_1 -> bv
        let s_58_4: Bits = Bits::new(s_58_1 as u128, 64u16);
        // C s_58_5: const #64s : i64
        let s_58_5: i64 = 64;
        // D s_58_6: call X_set(s_58_3, s_58_5, s_58_4)
        let s_58_6: () = X_set(state, tracer, s_58_3, s_58_5, s_58_4);
        // N s_58_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var accdesc:struct
        let s_59_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_59_1: call AlignmentFault(s_59_0)
        let s_59_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_59_0);
        // D s_59_2: read-var address:u64
        let s_59_2: u64 = fn_state.address;
        // D s_59_3: call AArch64_Abort(s_59_2, s_59_1)
        let s_59_3: () = AArch64_Abort(state, tracer, s_59_2, s_59_1);
        // N s_59_4: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var dbytes:i64
        let s_60_0: i64 = fn_state.dbytes;
        // D s_60_1: cast zx s_60_0 -> i
        let s_60_1: i128 = (i128::try_from(s_60_0).unwrap());
        // D s_60_2: read-var address:u64
        let s_60_2: u64 = fn_state.address;
        // D s_60_3: read-var accdesc:struct
        let s_60_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_60_4: call Mem_read(s_60_2, s_60_1, s_60_3)
        let s_60_4: Bits = Mem_read(state, tracer, s_60_2, s_60_1, s_60_3);
        // D s_60_5: write-var datashadow#1636 <= s_60_4
        fn_state.datashadow_1636 = s_60_4;
        // N s_60_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var accdesc.1:struct
        let s_61_0: u32 = fn_state.accdesc._1;
        // D s_61_1: call BigEndian(s_61_0)
        let s_61_1: bool = BigEndian(state, tracer, s_61_0);
        // N s_61_2: branch s_61_1 b69 b62
        if s_61_1 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var datasizeshadow#1634:i64
        let s_62_0: i64 = fn_state.datasizeshadow_1634;
        // D s_62_1: cast zx s_62_0 -> i
        let s_62_1: i128 = (i128::try_from(s_62_0).unwrap());
        // D s_62_2: call __id(s_62_1)
        let s_62_2: i128 = u__id(state, tracer, s_62_1);
        // D s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: const #31s : i
        let s_62_4: i128 = 31;
        // D s_62_5: cast zx s_62_3 -> i
        let s_62_5: i128 = (i128::try_from(s_62_3).unwrap());
        // D s_62_6: cmp-lt s_62_4 s_62_5
        let s_62_6: bool = ((s_62_4) < (s_62_5));
        // N s_62_7: assert s_62_6
        let s_62_7: () = assert!(s_62_6);
        // C s_62_8: const #32s : i64
        let s_62_8: i64 = 32;
        // C s_62_9: const #0s : i
        let s_62_9: i128 = 0;
        // C s_62_10: const #32s : i
        let s_62_10: i128 = 32;
        // D s_62_11: read-var datashadow#1636:bv
        let s_62_11: Bits = fn_state.datashadow_1636;
        // D s_62_12: bit-extract s_62_11 s_62_9 s_62_10
        let s_62_12: Bits = (Bits::new(
            ((s_62_11) >> (s_62_9)).value(),
            u16::try_from(s_62_10).unwrap(),
        ));
        // D s_62_13: cast reint s_62_12 -> u32
        let s_62_13: u32 = (s_62_12.value() as u32);
        // D s_62_14: read-var t:i64
        let s_62_14: i64 = fn_state.t;
        // D s_62_15: cast zx s_62_14 -> i
        let s_62_15: i128 = (i128::try_from(s_62_14).unwrap());
        // D s_62_16: cast zx s_62_13 -> bv
        let s_62_16: Bits = Bits::new(s_62_13 as u128, 32u16);
        // D s_62_17: call X_set(s_62_15, s_62_8, s_62_16)
        let s_62_17: () = X_set(state, tracer, s_62_15, s_62_8, s_62_16);
        // D s_62_18: read-var datasizeshadow#1634:i64
        let s_62_18: i64 = fn_state.datasizeshadow_1634;
        // D s_62_19: cast zx s_62_18 -> i
        let s_62_19: i128 = (i128::try_from(s_62_18).unwrap());
        // D s_62_20: call __id(s_62_19)
        let s_62_20: i128 = u__id(state, tracer, s_62_19);
        // D s_62_21: cast reint s_62_20 -> i64
        let s_62_21: i64 = (s_62_20 as i64);
        // C s_62_22: const #1s : i
        let s_62_22: i128 = 1;
        // D s_62_23: cast zx s_62_21 -> i
        let s_62_23: i128 = (i128::try_from(s_62_21).unwrap());
        // D s_62_24: sub s_62_23 s_62_22
        let s_62_24: i128 = ((s_62_23) - (s_62_22));
        // D s_62_25: cast reint s_62_24 -> i64
        let s_62_25: i64 = (s_62_24 as i64);
        // C s_62_26: const #32s : i
        let s_62_26: i128 = 32;
        // D s_62_27: cast zx s_62_25 -> i
        let s_62_27: i128 = (i128::try_from(s_62_25).unwrap());
        // D s_62_28: cmp-le s_62_26 s_62_27
        let s_62_28: bool = ((s_62_26) <= (s_62_27));
        // N s_62_29: branch s_62_28 b68 b63
        if s_62_28 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#159749 <= s_63_0
        fn_state.gs_159749 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#159749:u8
        let s_64_0: bool = fn_state.gs_159749;
        // N s_64_1: assert s_64_0
        let s_64_1: () = assert!(s_64_0);
        // D s_64_2: read-var datasizeshadow#1634:i64
        let s_64_2: i64 = fn_state.datasizeshadow_1634;
        // D s_64_3: cast zx s_64_2 -> i
        let s_64_3: i128 = (i128::try_from(s_64_2).unwrap());
        // D s_64_4: call __id(s_64_3)
        let s_64_4: i128 = u__id(state, tracer, s_64_3);
        // D s_64_5: cast reint s_64_4 -> i64
        let s_64_5: i64 = (s_64_4 as i64);
        // C s_64_6: const #32s : i
        let s_64_6: i128 = 32;
        // D s_64_7: cast zx s_64_5 -> i
        let s_64_7: i128 = (i128::try_from(s_64_5).unwrap());
        // D s_64_8: sub s_64_7 s_64_6
        let s_64_8: i128 = ((s_64_7) - (s_64_6));
        // D s_64_9: cast reint s_64_8 -> i64
        let s_64_9: i64 = (s_64_8 as i64);
        // C s_64_10: const #32s : i
        let s_64_10: i128 = 32;
        // D s_64_11: cast zx s_64_9 -> i
        let s_64_11: i128 = (i128::try_from(s_64_9).unwrap());
        // D s_64_12: cmp-eq s_64_11 s_64_10
        let s_64_12: bool = ((s_64_11) == (s_64_10));
        // N s_64_13: branch s_64_12 b67 b65
        if s_64_12 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var datasizeshadow#1634:i64
        let s_65_0: i64 = fn_state.datasizeshadow_1634;
        // D s_65_1: cast zx s_65_0 -> i
        let s_65_1: i128 = (i128::try_from(s_65_0).unwrap());
        // D s_65_2: call __id(s_65_1)
        let s_65_2: i128 = u__id(state, tracer, s_65_1);
        // D s_65_3: cast reint s_65_2 -> i64
        let s_65_3: i64 = (s_65_2 as i64);
        // C s_65_4: const #32s : i
        let s_65_4: i128 = 32;
        // D s_65_5: cast zx s_65_3 -> i
        let s_65_5: i128 = (i128::try_from(s_65_3).unwrap());
        // D s_65_6: sub s_65_5 s_65_4
        let s_65_6: i128 = ((s_65_5) - (s_65_4));
        // D s_65_7: cast reint s_65_6 -> i64
        let s_65_7: i64 = (s_65_6 as i64);
        // C s_65_8: const #64s : i
        let s_65_8: i128 = 64;
        // D s_65_9: cast zx s_65_7 -> i
        let s_65_9: i128 = (i128::try_from(s_65_7).unwrap());
        // D s_65_10: cmp-eq s_65_9 s_65_8
        let s_65_10: bool = ((s_65_9) == (s_65_8));
        // D s_65_11: write-var gs#159755 <= s_65_10
        fn_state.gs_159755 = s_65_10;
        // N s_65_12: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#159755:u8
        let s_66_0: bool = fn_state.gs_159755;
        // N s_66_1: assert s_66_0
        let s_66_1: () = assert!(s_66_0);
        // C s_66_2: const #32s : i
        let s_66_2: i128 = 32;
        // D s_66_3: read-var datasizeshadow#1634:i64
        let s_66_3: i64 = fn_state.datasizeshadow_1634;
        // D s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // D s_66_5: sub s_66_4 s_66_2
        let s_66_5: i128 = ((s_66_4) - (s_66_2));
        // D s_66_6: cast reint s_66_5 -> i64
        let s_66_6: i64 = (s_66_5 as i64);
        // D s_66_7: cast zx s_66_6 -> i
        let s_66_7: i128 = (i128::try_from(s_66_6).unwrap());
        // D s_66_8: cast reint s_66_7 -> i64
        let s_66_8: i64 = (s_66_7 as i64);
        // C s_66_9: const #32s : i
        let s_66_9: i128 = 32;
        // C s_66_10: const #32s : i
        let s_66_10: i128 = 32;
        // D s_66_11: read-var datashadow#1636:bv
        let s_66_11: Bits = fn_state.datashadow_1636;
        // D s_66_12: bit-extract s_66_11 s_66_9 s_66_10
        let s_66_12: Bits = (Bits::new(
            ((s_66_11) >> (s_66_9)).value(),
            u16::try_from(s_66_10).unwrap(),
        ));
        // D s_66_13: cast reint s_66_12 -> u32
        let s_66_13: u32 = (s_66_12.value() as u32);
        // D s_66_14: read-var t2:i64
        let s_66_14: i64 = fn_state.t2;
        // D s_66_15: cast zx s_66_14 -> i
        let s_66_15: i128 = (i128::try_from(s_66_14).unwrap());
        // D s_66_16: cast zx s_66_13 -> bv
        let s_66_16: Bits = Bits::new(s_66_13 as u128, 32u16);
        // D s_66_17: call X_set(s_66_15, s_66_8, s_66_16)
        let s_66_17: () = X_set(state, tracer, s_66_15, s_66_8, s_66_16);
        // N s_66_18: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#159755 <= s_67_0
        fn_state.gs_159755 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var datasizeshadow#1634:i64
        let s_68_0: i64 = fn_state.datasizeshadow_1634;
        // D s_68_1: cast zx s_68_0 -> i
        let s_68_1: i128 = (i128::try_from(s_68_0).unwrap());
        // D s_68_2: call __id(s_68_1)
        let s_68_2: i128 = u__id(state, tracer, s_68_1);
        // D s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: const #1s : i
        let s_68_4: i128 = 1;
        // D s_68_5: cast zx s_68_3 -> i
        let s_68_5: i128 = (i128::try_from(s_68_3).unwrap());
        // D s_68_6: sub s_68_5 s_68_4
        let s_68_6: i128 = ((s_68_5) - (s_68_4));
        // D s_68_7: cast reint s_68_6 -> i64
        let s_68_7: i64 = (s_68_6 as i64);
        // D s_68_8: read-var datasizeshadow#1634:i64
        let s_68_8: i64 = fn_state.datasizeshadow_1634;
        // D s_68_9: cast zx s_68_8 -> i
        let s_68_9: i128 = (i128::try_from(s_68_8).unwrap());
        // D s_68_10: call __id(s_68_9)
        let s_68_10: i128 = u__id(state, tracer, s_68_9);
        // D s_68_11: cast reint s_68_10 -> i64
        let s_68_11: i64 = (s_68_10 as i64);
        // D s_68_12: cast zx s_68_7 -> i
        let s_68_12: i128 = (i128::try_from(s_68_7).unwrap());
        // D s_68_13: cast zx s_68_11 -> i
        let s_68_13: i128 = (i128::try_from(s_68_11).unwrap());
        // D s_68_14: cmp-lt s_68_12 s_68_13
        let s_68_14: bool = ((s_68_12) < (s_68_13));
        // D s_68_15: write-var gs#159749 <= s_68_14
        fn_state.gs_159749 = s_68_14;
        // N s_68_16: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var datasizeshadow#1634:i64
        let s_69_0: i64 = fn_state.datasizeshadow_1634;
        // D s_69_1: cast zx s_69_0 -> i
        let s_69_1: i128 = (i128::try_from(s_69_0).unwrap());
        // D s_69_2: call __id(s_69_1)
        let s_69_2: i128 = u__id(state, tracer, s_69_1);
        // D s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: const #1s : i
        let s_69_4: i128 = 1;
        // D s_69_5: cast zx s_69_3 -> i
        let s_69_5: i128 = (i128::try_from(s_69_3).unwrap());
        // D s_69_6: sub s_69_5 s_69_4
        let s_69_6: i128 = ((s_69_5) - (s_69_4));
        // D s_69_7: cast reint s_69_6 -> i64
        let s_69_7: i64 = (s_69_6 as i64);
        // C s_69_8: const #32s : i
        let s_69_8: i128 = 32;
        // D s_69_9: cast zx s_69_7 -> i
        let s_69_9: i128 = (i128::try_from(s_69_7).unwrap());
        // D s_69_10: cmp-le s_69_8 s_69_9
        let s_69_10: bool = ((s_69_8) <= (s_69_9));
        // N s_69_11: branch s_69_10 b75 b70
        if s_69_10 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#159766 <= s_70_0
        fn_state.gs_159766 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#159766:u8
        let s_71_0: bool = fn_state.gs_159766;
        // N s_71_1: assert s_71_0
        let s_71_1: () = assert!(s_71_0);
        // D s_71_2: read-var datasizeshadow#1634:i64
        let s_71_2: i64 = fn_state.datasizeshadow_1634;
        // D s_71_3: cast zx s_71_2 -> i
        let s_71_3: i128 = (i128::try_from(s_71_2).unwrap());
        // D s_71_4: call __id(s_71_3)
        let s_71_4: i128 = u__id(state, tracer, s_71_3);
        // D s_71_5: cast reint s_71_4 -> i64
        let s_71_5: i64 = (s_71_4 as i64);
        // C s_71_6: const #32s : i
        let s_71_6: i128 = 32;
        // D s_71_7: cast zx s_71_5 -> i
        let s_71_7: i128 = (i128::try_from(s_71_5).unwrap());
        // D s_71_8: sub s_71_7 s_71_6
        let s_71_8: i128 = ((s_71_7) - (s_71_6));
        // D s_71_9: cast reint s_71_8 -> i64
        let s_71_9: i64 = (s_71_8 as i64);
        // C s_71_10: const #32s : i
        let s_71_10: i128 = 32;
        // D s_71_11: cast zx s_71_9 -> i
        let s_71_11: i128 = (i128::try_from(s_71_9).unwrap());
        // D s_71_12: cmp-eq s_71_11 s_71_10
        let s_71_12: bool = ((s_71_11) == (s_71_10));
        // N s_71_13: branch s_71_12 b74 b72
        if s_71_12 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var datasizeshadow#1634:i64
        let s_72_0: i64 = fn_state.datasizeshadow_1634;
        // D s_72_1: cast zx s_72_0 -> i
        let s_72_1: i128 = (i128::try_from(s_72_0).unwrap());
        // D s_72_2: call __id(s_72_1)
        let s_72_2: i128 = u__id(state, tracer, s_72_1);
        // D s_72_3: cast reint s_72_2 -> i64
        let s_72_3: i64 = (s_72_2 as i64);
        // C s_72_4: const #32s : i
        let s_72_4: i128 = 32;
        // D s_72_5: cast zx s_72_3 -> i
        let s_72_5: i128 = (i128::try_from(s_72_3).unwrap());
        // D s_72_6: sub s_72_5 s_72_4
        let s_72_6: i128 = ((s_72_5) - (s_72_4));
        // D s_72_7: cast reint s_72_6 -> i64
        let s_72_7: i64 = (s_72_6 as i64);
        // C s_72_8: const #64s : i
        let s_72_8: i128 = 64;
        // D s_72_9: cast zx s_72_7 -> i
        let s_72_9: i128 = (i128::try_from(s_72_7).unwrap());
        // D s_72_10: cmp-eq s_72_9 s_72_8
        let s_72_10: bool = ((s_72_9) == (s_72_8));
        // D s_72_11: write-var gs#159772 <= s_72_10
        fn_state.gs_159772 = s_72_10;
        // N s_72_12: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#159772:u8
        let s_73_0: bool = fn_state.gs_159772;
        // N s_73_1: assert s_73_0
        let s_73_1: () = assert!(s_73_0);
        // C s_73_2: const #32s : i
        let s_73_2: i128 = 32;
        // D s_73_3: read-var datasizeshadow#1634:i64
        let s_73_3: i64 = fn_state.datasizeshadow_1634;
        // D s_73_4: cast zx s_73_3 -> i
        let s_73_4: i128 = (i128::try_from(s_73_3).unwrap());
        // D s_73_5: sub s_73_4 s_73_2
        let s_73_5: i128 = ((s_73_4) - (s_73_2));
        // D s_73_6: cast reint s_73_5 -> i64
        let s_73_6: i64 = (s_73_5 as i64);
        // D s_73_7: cast zx s_73_6 -> i
        let s_73_7: i128 = (i128::try_from(s_73_6).unwrap());
        // D s_73_8: cast reint s_73_7 -> i64
        let s_73_8: i64 = (s_73_7 as i64);
        // C s_73_9: const #32s : i
        let s_73_9: i128 = 32;
        // C s_73_10: const #32s : i
        let s_73_10: i128 = 32;
        // D s_73_11: read-var datashadow#1636:bv
        let s_73_11: Bits = fn_state.datashadow_1636;
        // D s_73_12: bit-extract s_73_11 s_73_9 s_73_10
        let s_73_12: Bits = (Bits::new(
            ((s_73_11) >> (s_73_9)).value(),
            u16::try_from(s_73_10).unwrap(),
        ));
        // D s_73_13: cast reint s_73_12 -> u32
        let s_73_13: u32 = (s_73_12.value() as u32);
        // D s_73_14: read-var t:i64
        let s_73_14: i64 = fn_state.t;
        // D s_73_15: cast zx s_73_14 -> i
        let s_73_15: i128 = (i128::try_from(s_73_14).unwrap());
        // D s_73_16: cast zx s_73_13 -> bv
        let s_73_16: Bits = Bits::new(s_73_13 as u128, 32u16);
        // D s_73_17: call X_set(s_73_15, s_73_8, s_73_16)
        let s_73_17: () = X_set(state, tracer, s_73_15, s_73_8, s_73_16);
        // C s_73_18: const #32s : i64
        let s_73_18: i64 = 32;
        // C s_73_19: const #0s : i
        let s_73_19: i128 = 0;
        // C s_73_20: const #32s : i
        let s_73_20: i128 = 32;
        // D s_73_21: read-var datashadow#1636:bv
        let s_73_21: Bits = fn_state.datashadow_1636;
        // D s_73_22: bit-extract s_73_21 s_73_19 s_73_20
        let s_73_22: Bits = (Bits::new(
            ((s_73_21) >> (s_73_19)).value(),
            u16::try_from(s_73_20).unwrap(),
        ));
        // D s_73_23: cast reint s_73_22 -> u32
        let s_73_23: u32 = (s_73_22.value() as u32);
        // D s_73_24: read-var t2:i64
        let s_73_24: i64 = fn_state.t2;
        // D s_73_25: cast zx s_73_24 -> i
        let s_73_25: i128 = (i128::try_from(s_73_24).unwrap());
        // D s_73_26: cast zx s_73_23 -> bv
        let s_73_26: Bits = Bits::new(s_73_23 as u128, 32u16);
        // D s_73_27: call X_set(s_73_25, s_73_18, s_73_26)
        let s_73_27: () = X_set(state, tracer, s_73_25, s_73_18, s_73_26);
        // N s_73_28: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#159772 <= s_74_0
        fn_state.gs_159772 = s_74_0;
        // N s_74_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var datasizeshadow#1634:i64
        let s_75_0: i64 = fn_state.datasizeshadow_1634;
        // D s_75_1: cast zx s_75_0 -> i
        let s_75_1: i128 = (i128::try_from(s_75_0).unwrap());
        // D s_75_2: call __id(s_75_1)
        let s_75_2: i128 = u__id(state, tracer, s_75_1);
        // D s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: const #1s : i
        let s_75_4: i128 = 1;
        // D s_75_5: cast zx s_75_3 -> i
        let s_75_5: i128 = (i128::try_from(s_75_3).unwrap());
        // D s_75_6: sub s_75_5 s_75_4
        let s_75_6: i128 = ((s_75_5) - (s_75_4));
        // D s_75_7: cast reint s_75_6 -> i64
        let s_75_7: i64 = (s_75_6 as i64);
        // D s_75_8: read-var datasizeshadow#1634:i64
        let s_75_8: i64 = fn_state.datasizeshadow_1634;
        // D s_75_9: cast zx s_75_8 -> i
        let s_75_9: i128 = (i128::try_from(s_75_8).unwrap());
        // D s_75_10: call __id(s_75_9)
        let s_75_10: i128 = u__id(state, tracer, s_75_9);
        // D s_75_11: cast reint s_75_10 -> i64
        let s_75_11: i64 = (s_75_10 as i64);
        // D s_75_12: cast zx s_75_7 -> i
        let s_75_12: i128 = (i128::try_from(s_75_7).unwrap());
        // D s_75_13: cast zx s_75_11 -> i
        let s_75_13: i128 = (i128::try_from(s_75_11).unwrap());
        // D s_75_14: cmp-lt s_75_12 s_75_13
        let s_75_14: bool = ((s_75_12) < (s_75_13));
        // D s_75_15: write-var gs#159766 <= s_75_14
        fn_state.gs_159766 = s_75_14;
        // N s_75_16: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var datasizeshadow#1634:i64
        let s_76_0: i64 = fn_state.datasizeshadow_1634;
        // D s_76_1: cast zx s_76_0 -> i
        let s_76_1: i128 = (i128::try_from(s_76_0).unwrap());
        // D s_76_2: call __id(s_76_1)
        let s_76_2: i128 = u__id(state, tracer, s_76_1);
        // D s_76_3: cast reint s_76_2 -> i64
        let s_76_3: i64 = (s_76_2 as i64);
        // C s_76_4: const #32s : i
        let s_76_4: i128 = 32;
        // D s_76_5: cast zx s_76_3 -> i
        let s_76_5: i128 = (i128::try_from(s_76_3).unwrap());
        // D s_76_6: cmp-eq s_76_5 s_76_4
        let s_76_6: bool = ((s_76_5) == (s_76_4));
        // N s_76_7: branch s_76_6 b79 b77
        if s_76_6 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var datasizeshadow#1634:i64
        let s_77_0: i64 = fn_state.datasizeshadow_1634;
        // D s_77_1: cast zx s_77_0 -> i
        let s_77_1: i128 = (i128::try_from(s_77_0).unwrap());
        // D s_77_2: call __id(s_77_1)
        let s_77_2: i128 = u__id(state, tracer, s_77_1);
        // D s_77_3: cast reint s_77_2 -> i64
        let s_77_3: i64 = (s_77_2 as i64);
        // C s_77_4: const #64s : i
        let s_77_4: i128 = 64;
        // D s_77_5: cast zx s_77_3 -> i
        let s_77_5: i128 = (i128::try_from(s_77_3).unwrap());
        // D s_77_6: cmp-eq s_77_5 s_77_4
        let s_77_6: bool = ((s_77_5) == (s_77_4));
        // D s_77_7: write-var gs#159784 <= s_77_6
        fn_state.gs_159784 = s_77_6;
        // N s_77_8: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#159784:u8
        let s_78_0: bool = fn_state.gs_159784;
        // N s_78_1: assert s_78_0
        let s_78_1: () = assert!(s_78_0);
        // D s_78_2: read-var datasizeshadow#1634:i64
        let s_78_2: i64 = fn_state.datasizeshadow_1634;
        // D s_78_3: cast zx s_78_2 -> i
        let s_78_3: i128 = (i128::try_from(s_78_2).unwrap());
        // D s_78_4: cast reint s_78_3 -> i64
        let s_78_4: i64 = (s_78_3 as i64);
        // D s_78_5: read-var datasizeshadow#1634:i64
        let s_78_5: i64 = fn_state.datasizeshadow_1634;
        // D s_78_6: cast zx s_78_5 -> i
        let s_78_6: i128 = (i128::try_from(s_78_5).unwrap());
        // D s_78_7: cast reint s_78_6 -> i64
        let s_78_7: i64 = (s_78_6 as i64);
        // D s_78_8: cast zx s_78_7 -> i
        let s_78_8: i128 = (i128::try_from(s_78_7).unwrap());
        // D s_78_9: call __UNKNOWN_bits(s_78_8)
        let s_78_9: Bits = u__UNKNOWN_bits(state, tracer, s_78_8);
        // D s_78_10: read-var t:i64
        let s_78_10: i64 = fn_state.t;
        // D s_78_11: cast zx s_78_10 -> i
        let s_78_11: i128 = (i128::try_from(s_78_10).unwrap());
        // D s_78_12: call X_set(s_78_11, s_78_4, s_78_9)
        let s_78_12: () = X_set(state, tracer, s_78_11, s_78_4, s_78_9);
        // N s_78_13: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var gs#159784 <= s_79_0
        fn_state.gs_159784 = s_79_0;
        // N s_79_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_80_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #64s : i64
        let s_81_0: i64 = 64;
        // C s_81_1: cast zx s_81_0 -> i
        let s_81_1: i128 = (i128::try_from(s_81_0).unwrap());
        // S s_81_2: call __UNKNOWN_bits(s_81_1)
        let s_81_2: Bits = u__UNKNOWN_bits(state, tracer, s_81_1);
        // S s_81_3: cast reint s_81_2 -> u64
        let s_81_3: u64 = (s_81_2.value() as u64);
        // D s_81_4: write-var address <= s_81_3
        fn_state.address = s_81_3;
        // N s_81_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call CheckSPAlignment(s_82_0)
        let s_82_1: () = CheckSPAlignment(state, tracer, s_82_0);
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call SP_read(s_83_0)
        let s_83_1: u64 = SP_read(state, tracer, s_83_0);
        // D s_83_2: write-var address <= s_83_1
        fn_state.address = s_83_1;
        // N s_83_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
