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
use SP_read::*;
use SPESampleSIMDFPLoadStore::*;
use X_read::*;
use CreateAccDescASIMD::*;
use CheckSPAlignment::*;
use Mem_set::*;
use common::*;
pub fn execute_aarch64_instrs_memory_single_simdfp_immediate_signed_post_idx<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    memop: u32,
    n: i64,
    nontemporal: bool,
    offset: u64,
    postindex: bool,
    t: i64,
    tagchecked: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_161566: bool,
        gs_161568: bool,
        datasizeshadow_1649: i64,
        gs_161575: bool,
        address: u64,
        gs_161562: bool,
        gs_161579: bool,
        gs_161581: bool,
        datashadow_1651: Bits,
        gs_161577: bool,
        accdesc: ProductType9878976b5bcce9c9,
        gs_161564: bool,
        datasize: i64,
        memop: u32,
        n: i64,
        nontemporal: bool,
        offset: u64,
        postindex: bool,
        t: i64,
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
        t,
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
        // D s_0_3: write-var datasizeshadow#1649 <= s_0_2
        fn_state.datasizeshadow_1649 = s_0_2;
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
        // D s_1_0: read-var memop:u32
        let s_1_0: u32 = fn_state.memop;
        // D s_1_1: read-var nontemporal:u8
        let s_1_1: bool = fn_state.nontemporal;
        // D s_1_2: read-var tagchecked:u8
        let s_1_2: bool = fn_state.tagchecked;
        // D s_1_3: call CreateAccDescASIMD(s_1_0, s_1_1, s_1_2)
        let s_1_3: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_0,
            s_1_1,
            s_1_2,
        );
        // D s_1_4: write-var accdesc <= s_1_3
        fn_state.accdesc = s_1_3;
        // C s_1_5: const #31s : i
        let s_1_5: i128 = 31;
        // D s_1_6: read-var n:i64
        let s_1_6: i64 = fn_state.n;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: cmp-eq s_1_7 s_1_5
        let s_1_8: bool = ((s_1_7) == (s_1_5));
        // N s_1_9: branch s_1_8 b47 b2
        if s_1_8 {
            return block_47(state, tracer, fn_state);
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
        // N s_3_2: branch s_3_1 b46 b4
        if s_3_1 {
            return block_46(state, tracer, fn_state);
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
        // N s_5_4: branch s_5_3 b30 b6
        if s_5_3 {
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
        // D s_6_0: read-var datasizeshadow#1649:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1649;
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
        // D s_7_0: read-var datasizeshadow#1649:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1649;
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
        // D s_7_7: write-var gs#161562 <= s_7_6
        fn_state.gs_161562 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#161562:u8
        let s_8_0: bool = fn_state.gs_161562;
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
        // D s_9_0: read-var datasizeshadow#1649:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1649;
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
        // D s_9_7: write-var gs#161564 <= s_9_6
        fn_state.gs_161564 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#161564:u8
        let s_10_0: bool = fn_state.gs_161564;
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
        // D s_11_0: read-var datasizeshadow#1649:i64
        let s_11_0: i64 = fn_state.datasizeshadow_1649;
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
        // D s_11_7: write-var gs#161566 <= s_11_6
        fn_state.gs_161566 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#161566:u8
        let s_12_0: bool = fn_state.gs_161566;
        // N s_12_1: branch s_12_0 b26 b13
        if s_12_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var datasizeshadow#1649:i64
        let s_13_0: i64 = fn_state.datasizeshadow_1649;
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
        // D s_13_7: write-var gs#161568 <= s_13_6
        fn_state.gs_161568 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#161568:u8
        let s_14_0: bool = fn_state.gs_161568;
        // N s_14_1: assert s_14_0
        let s_14_1: () = assert!(s_14_0);
        // D s_14_2: read-var datasizeshadow#1649:i64
        let s_14_2: i64 = fn_state.datasizeshadow_1649;
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
        // C s_14_8: const #8s : i
        let s_14_8: i128 = 8;
        // D s_14_9: read-var datasizeshadow#1649:i64
        let s_14_9: i64 = fn_state.datasizeshadow_1649;
        // D s_14_10: cast zx s_14_9 -> i
        let s_14_10: i128 = (i128::try_from(s_14_9).unwrap());
        // D s_14_11: div s_14_10 s_14_8
        let s_14_11: i128 = ((s_14_10) / (s_14_8));
        // D s_14_12: cast reint s_14_11 -> i64
        let s_14_12: i64 = (s_14_11 as i64);
        // D s_14_13: cast zx s_14_12 -> i
        let s_14_13: i128 = (i128::try_from(s_14_12).unwrap());
        // D s_14_14: read-var address:u64
        let s_14_14: u64 = fn_state.address;
        // D s_14_15: read-var accdesc:struct
        let s_14_15: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_16: call Mem_set(s_14_14, s_14_13, s_14_15, s_14_7)
        let s_14_16: () = Mem_set(state, tracer, s_14_14, s_14_13, s_14_15, s_14_7);
        // N s_14_17: jump b15
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
        // S s_19_1: call SPESampleSIMDFPLoadStore(s_19_0)
        let s_19_1: () = SPESampleSIMDFPLoadStore(state, tracer, s_19_0);
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
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#161568 <= s_26_0
        fn_state.gs_161568 = s_26_0;
        // N s_26_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#161566 <= s_27_0
        fn_state.gs_161566 = s_27_0;
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
        // D s_28_1: write-var gs#161564 <= s_28_0
        fn_state.gs_161564 = s_28_0;
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
        // D s_29_1: write-var gs#161562 <= s_29_0
        fn_state.gs_161562 = s_29_0;
        // N s_29_2: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_30_4: branch s_30_3 b45 b31
        if s_30_3 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #8s : i
        let s_31_0: i128 = 8;
        // D s_31_1: read-var datasizeshadow#1649:i64
        let s_31_1: i64 = fn_state.datasizeshadow_1649;
        // D s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (i128::try_from(s_31_1).unwrap());
        // D s_31_3: div s_31_2 s_31_0
        let s_31_3: i128 = ((s_31_2) / (s_31_0));
        // D s_31_4: cast reint s_31_3 -> i64
        let s_31_4: i64 = (s_31_3 as i64);
        // D s_31_5: cast zx s_31_4 -> i
        let s_31_5: i128 = (i128::try_from(s_31_4).unwrap());
        // D s_31_6: read-var address:u64
        let s_31_6: u64 = fn_state.address;
        // D s_31_7: read-var accdesc:struct
        let s_31_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_31_8: call Mem_read(s_31_6, s_31_5, s_31_7)
        let s_31_8: Bits = Mem_read(state, tracer, s_31_6, s_31_5, s_31_7);
        // D s_31_9: write-var datashadow#1651 <= s_31_8
        fn_state.datashadow_1651 = s_31_8;
        // N s_31_10: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var datasizeshadow#1649:i64
        let s_32_0: i64 = fn_state.datasizeshadow_1649;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call __id(s_32_1)
        let s_32_2: i128 = u__id(state, tracer, s_32_1);
        // D s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: const #8s : i
        let s_32_4: i128 = 8;
        // D s_32_5: cast zx s_32_3 -> i
        let s_32_5: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_6: cmp-eq s_32_5 s_32_4
        let s_32_6: bool = ((s_32_5) == (s_32_4));
        // N s_32_7: branch s_32_6 b44 b33
        if s_32_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var datasizeshadow#1649:i64
        let s_33_0: i64 = fn_state.datasizeshadow_1649;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #16s : i
        let s_33_4: i128 = 16;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-eq s_33_5 s_33_4
        let s_33_6: bool = ((s_33_5) == (s_33_4));
        // D s_33_7: write-var gs#161575 <= s_33_6
        fn_state.gs_161575 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#161575:u8
        let s_34_0: bool = fn_state.gs_161575;
        // N s_34_1: branch s_34_0 b43 b35
        if s_34_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var datasizeshadow#1649:i64
        let s_35_0: i64 = fn_state.datasizeshadow_1649;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: call __id(s_35_1)
        let s_35_2: i128 = u__id(state, tracer, s_35_1);
        // D s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: const #32s : i
        let s_35_4: i128 = 32;
        // D s_35_5: cast zx s_35_3 -> i
        let s_35_5: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_6: cmp-eq s_35_5 s_35_4
        let s_35_6: bool = ((s_35_5) == (s_35_4));
        // D s_35_7: write-var gs#161577 <= s_35_6
        fn_state.gs_161577 = s_35_6;
        // N s_35_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#161577:u8
        let s_36_0: bool = fn_state.gs_161577;
        // N s_36_1: branch s_36_0 b42 b37
        if s_36_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var datasizeshadow#1649:i64
        let s_37_0: i64 = fn_state.datasizeshadow_1649;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: call __id(s_37_1)
        let s_37_2: i128 = u__id(state, tracer, s_37_1);
        // D s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: const #64s : i
        let s_37_4: i128 = 64;
        // D s_37_5: cast zx s_37_3 -> i
        let s_37_5: i128 = (i128::try_from(s_37_3).unwrap());
        // D s_37_6: cmp-eq s_37_5 s_37_4
        let s_37_6: bool = ((s_37_5) == (s_37_4));
        // D s_37_7: write-var gs#161579 <= s_37_6
        fn_state.gs_161579 = s_37_6;
        // N s_37_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#161579:u8
        let s_38_0: bool = fn_state.gs_161579;
        // N s_38_1: branch s_38_0 b41 b39
        if s_38_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var datasizeshadow#1649:i64
        let s_39_0: i64 = fn_state.datasizeshadow_1649;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call __id(s_39_1)
        let s_39_2: i128 = u__id(state, tracer, s_39_1);
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #128s : i
        let s_39_4: i128 = 128;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: cmp-eq s_39_5 s_39_4
        let s_39_6: bool = ((s_39_5) == (s_39_4));
        // D s_39_7: write-var gs#161581 <= s_39_6
        fn_state.gs_161581 = s_39_6;
        // N s_39_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#161581:u8
        let s_40_0: bool = fn_state.gs_161581;
        // N s_40_1: assert s_40_0
        let s_40_1: () = assert!(s_40_0);
        // D s_40_2: read-var datasizeshadow#1649:i64
        let s_40_2: i64 = fn_state.datasizeshadow_1649;
        // D s_40_3: cast zx s_40_2 -> i
        let s_40_3: i128 = (i128::try_from(s_40_2).unwrap());
        // D s_40_4: cast reint s_40_3 -> i64
        let s_40_4: i64 = (s_40_3 as i64);
        // D s_40_5: read-var t:i64
        let s_40_5: i64 = fn_state.t;
        // D s_40_6: cast zx s_40_5 -> i
        let s_40_6: i128 = (i128::try_from(s_40_5).unwrap());
        // D s_40_7: read-var datashadow#1651:bv
        let s_40_7: Bits = fn_state.datashadow_1651;
        // D s_40_8: call V_set(s_40_6, s_40_4, s_40_7)
        let s_40_8: () = V_set(state, tracer, s_40_6, s_40_4, s_40_7);
        // N s_40_9: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#161581 <= s_41_0
        fn_state.gs_161581 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#161579 <= s_42_0
        fn_state.gs_161579 = s_42_0;
        // N s_42_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#161577 <= s_43_0
        fn_state.gs_161577 = s_43_0;
        // N s_43_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#161575 <= s_44_0
        fn_state.gs_161575 = s_44_0;
        // N s_44_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var address:u64
        let s_46_0: u64 = fn_state.address;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 64u16);
        // D s_46_2: read-var offset:u64
        let s_46_2: u64 = fn_state.offset;
        // D s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 64u16);
        // D s_46_4: add s_46_1 s_46_3
        let s_46_4: Bits = (s_46_1 + s_46_3);
        // D s_46_5: cast reint s_46_4 -> u64
        let s_46_5: u64 = (s_46_4.value() as u64);
        // D s_46_6: write-var address <= s_46_5
        fn_state.address = s_46_5;
        // N s_46_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call CheckSPAlignment(s_47_0)
        let s_47_1: () = CheckSPAlignment(state, tracer, s_47_0);
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call SP_read(s_48_0)
        let s_48_1: u64 = SP_read(state, tracer, s_48_0);
        // D s_48_2: write-var address <= s_48_1
        fn_state.address = s_48_1;
        // N s_48_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
