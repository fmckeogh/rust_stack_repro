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
use IsFault__2::*;
use FailTransaction::*;
use HaveMTE2Ext::*;
use HaveLSE2Ext::*;
use ConstrainUnpredictableBool::*;
use AllInAlignedQuantity::*;
use AArch64_PhysicalTag::*;
use AArch64_TagCheckFault::*;
use IsAligned__1::*;
use u__IMPDEF_boolean::*;
use SPESampleLoadStore::*;
use MemHasTransactionalAccess::*;
use ProcessorID::*;
use PhysMemWrite::*;
use AArch64_TranslateAddress::*;
use HaveTME::*;
use ClearExclusiveByAddress::*;
use u__id::*;
use HandleExternalWriteAbort::*;
use AArch64_AccessIsTagChecked::*;
use AArch64_Abort::*;
use IsFault::*;
use AlignmentFault::*;
use AArch64_CheckTag::*;
use common::*;
pub fn AArch64_MemSingle_set__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i64,
    accdesc_in: ProductType9878976b5bcce9c9,
    aligned: bool,
    ispair: bool,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_20186: bool,
        gs_20224: bool,
        gs_20249: bool,
        gs_20204: bool,
        gs_20189: bool,
        gs_20250: bool,
        gs_20195: bool,
        ga_15678: ProductTypeda0231e9dc169f81,
        gs_20207: bool,
        ga_15686: ProductTypeda0231e9dc169f81,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        halfsize: i64,
        ga_15626: bool,
        ga_15634: ProductTypef170cab34335b70c,
        gs_20251: bool,
        memstatusshadow_326: ProductTypef8c3639b88223255,
        i: i64,
        gs_20248: bool,
        gs_20205: bool,
        gs_20200: bool,
        gs_20211: i64,
        memstatusshadow_325: ProductTypef8c3639b88223255,
        ga_15614: ProductTypef170cab34335b70c,
        ga_15635: ProductType183e6678e5239c85,
        gs_20247: bool,
        highhalf: Bits,
        gs_20187: bool,
        memstatus: ProductTypef8c3639b88223255,
        ga_15631: ProductType183e6678e5239c85,
        accdesc: ProductType9878976b5bcce9c9,
        gs_20188: bool,
        ga_15628: ProductTypef170cab34335b70c,
        atomic: bool,
        gs_20191: bool,
        ga_15630: ProductTypef170cab34335b70c,
        address: u64,
        size: i64,
        accdesc_in: ProductType9878976b5bcce9c9,
        aligned: bool,
        ispair: bool,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc_in,
        aligned,
        ispair,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var size:i64
        let s_0_1: i64 = fn_state.size;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b103 b1
        if s_0_3 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var size:i64
        let s_1_1: i64 = fn_state.size;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_0
        let s_1_3: bool = ((s_1_2) == (s_1_0));
        // N s_1_4: branch s_1_3 b102 b2
        if s_1_3 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var size:i64
        let s_2_1: i64 = fn_state.size;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b101 b3
        if s_2_3 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var size:i64
        let s_3_1: i64 = fn_state.size;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b100 b4
        if s_3_3 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16s : i
        let s_4_0: i128 = 16;
        // D s_4_1: read-var size:i64
        let s_4_1: i64 = fn_state.size;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // D s_4_4: write-var gs#20186 <= s_4_3
        fn_state.gs_20186 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#20186:u8
        let s_5_0: bool = fn_state.gs_20186;
        // D s_5_1: write-var gs#20187 <= s_5_0
        fn_state.gs_20187 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#20187:u8
        let s_6_0: bool = fn_state.gs_20187;
        // D s_6_1: write-var gs#20188 <= s_6_0
        fn_state.gs_20188 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#20188:u8
        let s_7_0: bool = fn_state.gs_20188;
        // D s_7_1: write-var gs#20189 <= s_7_0
        fn_state.gs_20189 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#20189:u8
        let s_8_0: bool = fn_state.gs_20189;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // D s_8_2: read-var accdesc_in:struct
        let s_8_2: ProductType9878976b5bcce9c9 = fn_state.accdesc_in;
        // D s_8_3: write-var accdesc <= s_8_2
        fn_state.accdesc = s_8_2;
        // C s_8_4: const #() : ()
        let s_8_4: () = ();
        // S s_8_5: call HaveLSE2Ext(s_8_4)
        let s_8_5: bool = HaveLSE2Ext(state, tracer, s_8_4);
        // N s_8_6: branch s_8_5 b99 b9
        if s_8_5 {
            return block_99(state, tracer, fn_state);
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
        // D s_9_2: read-var size:i64
        let s_9_2: i64 = fn_state.size;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: call IsAligned__1(s_9_1, s_9_3)
        let s_9_4: bool = IsAligned__1(state, tracer, s_9_1, s_9_3);
        // N s_9_5: assert s_9_4
        let s_9_5: () = assert!(s_9_4);
        // N s_9_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HaveMTE2Ext(s_10_0)
        let s_10_1: bool = HaveMTE2Ext(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b98 b11
        if s_10_1 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#20191 <= s_11_0
        fn_state.gs_20191 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#20191:u8
        let s_12_0: bool = fn_state.gs_20191;
        // N s_12_1: branch s_12_0 b97 b13
        if s_12_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var size:i64
        let s_14_0: i64 = fn_state.size;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var address:u64
        let s_14_2: u64 = fn_state.address;
        // D s_14_3: read-var accdesc:struct
        let s_14_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_4: read-var aligned:u8
        let s_14_4: bool = fn_state.aligned;
        // D s_14_5: call AArch64_TranslateAddress(s_14_2, s_14_3, s_14_4, s_14_1)
        let s_14_5: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_14_2,
            s_14_3,
            s_14_4,
            s_14_1,
        );
        // D s_14_6: write-var memaddrdesc <= s_14_5
        fn_state.memaddrdesc = s_14_5;
        // N s_14_7: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var memaddrdesc:struct
        let s_15_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_15_1: call IsFault(s_15_0)
        let s_15_1: bool = IsFault(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b96 b16
        if s_15_1 {
            return block_96(state, tracer, fn_state);
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
        // D s_17_0: read-var memaddrdesc.2:struct
        let s_17_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_17_1: write-var ga#15614 <= s_17_0
        fn_state.ga_15614 = s_17_0;
        // D s_17_2: read-var ga#15614.5:struct
        let s_17_2: u32 = fn_state.ga_15614._5;
        // C s_17_3: const #0u : u32
        let s_17_3: u32 = 0;
        // D s_17_4: cmp-eq s_17_2 s_17_3
        let s_17_4: bool = ((s_17_2) == (s_17_3));
        // N s_17_5: branch s_17_4 b95 b18
        if s_17_4 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveTME(s_19_0)
        let s_19_1: bool = HaveTME(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b88 b20
        if s_19_1 {
            return block_88(state, tracer, fn_state);
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
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HaveMTE2Ext(s_21_0)
        let s_21_1: bool = HaveMTE2Ext(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b87 b22
        if s_21_1 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#20195 <= s_22_0
        fn_state.gs_20195 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#20195:u8
        let s_23_0: bool = fn_state.gs_20195;
        // N s_23_1: branch s_23_0 b82 b24
        if s_23_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #22416u : u32
        let s_25_0: u32 = 22416;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: bool = {
            let value = state.read_register::<bool>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // N s_25_2: branch s_25_1 b81 b26
        if s_25_1 {
            return block_81(state, tracer, fn_state);
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
        // D s_27_0: read-var memaddrdesc.2:struct
        let s_27_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_27_1: write-var ga#15628 <= s_27_0
        fn_state.ga_15628 = s_27_0;
        // D s_27_2: read-var ga#15628.2:struct
        let s_27_2: u32 = fn_state.ga_15628._2;
        // C s_27_3: const #0u : u32
        let s_27_3: u32 = 0;
        // D s_27_4: cmp-eq s_27_2 s_27_3
        let s_27_4: bool = ((s_27_2) == (s_27_3));
        // N s_27_5: branch s_27_4 b80 b28
        if s_27_4 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#20204 <= s_28_0
        fn_state.gs_20204 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#20204:u8
        let s_29_0: bool = fn_state.gs_20204;
        // N s_29_1: branch s_29_0 b79 b30
        if s_29_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#20205 <= s_30_0
        fn_state.gs_20205 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#20205:u8
        let s_31_0: bool = fn_state.gs_20205;
        // N s_31_1: branch s_31_0 b78 b32
        if s_31_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var accdesc.9:struct
        let s_32_0: bool = fn_state.accdesc._9;
        // N s_32_1: branch s_32_0 b77 b33
        if s_32_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var accdesc.4:struct
        let s_33_0: bool = fn_state.accdesc._4;
        // D s_33_1: write-var gs#20247 <= s_33_0
        fn_state.gs_20247 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#20247:u8
        let s_34_0: bool = fn_state.gs_20247;
        // N s_34_1: branch s_34_0 b76 b35
        if s_34_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var accdesc.3:struct
        let s_35_0: bool = fn_state.accdesc._3;
        // D s_35_1: write-var gs#20248 <= s_35_0
        fn_state.gs_20248 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#20248:u8
        let s_36_0: bool = fn_state.gs_20248;
        // N s_36_1: branch s_36_0 b75 b37
        if s_36_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var accdesc.2:struct
        let s_37_0: bool = fn_state.accdesc._2;
        // D s_37_1: write-var gs#20249 <= s_37_0
        fn_state.gs_20249 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#20249:u8
        let s_38_0: bool = fn_state.gs_20249;
        // N s_38_1: branch s_38_0 b74 b39
        if s_38_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var accdesc.24:struct
        let s_39_0: bool = fn_state.accdesc._24;
        // D s_39_1: write-var gs#20250 <= s_39_0
        fn_state.gs_20250 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#20250:u8
        let s_40_0: bool = fn_state.gs_20250;
        // N s_40_1: branch s_40_0 b68 b41
        if s_40_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var aligned:u8
        let s_41_0: bool = fn_state.aligned;
        // N s_41_1: branch s_41_0 b67 b42
        if s_41_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #"FEAT_LSE2: access is atomic" : str
        let s_42_0: &'static str = "FEAT_LSE2: access is atomic";
        // S s_42_1: call __IMPDEF_boolean(s_42_0)
        let s_42_1: bool = u__IMPDEF_boolean(state, tracer, s_42_0);
        // D s_42_2: write-var atomic <= s_42_1
        fn_state.atomic = s_42_1;
        // N s_42_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var atomic:u8
        let s_43_0: bool = fn_state.atomic;
        // N s_43_1: branch s_43_0 b64 b44
        if s_43_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var aligned:u8
        let s_44_0: bool = fn_state.aligned;
        // N s_44_1: branch s_44_0 b63 b45
        if s_44_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#20207 <= s_45_0
        fn_state.gs_20207 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#20207:u8
        let s_46_0: bool = fn_state.gs_20207;
        // N s_46_1: branch s_46_0 b54 b47
        if s_46_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0s : i64
        let s_47_0: i64 = 0;
        // C s_47_1: const #1s : i
        let s_47_1: i128 = 1;
        // D s_47_2: read-var size:i64
        let s_47_2: i64 = fn_state.size;
        // D s_47_3: cast zx s_47_2 -> i
        let s_47_3: i128 = (i128::try_from(s_47_2).unwrap());
        // D s_47_4: sub s_47_3 s_47_1
        let s_47_4: i128 = ((s_47_3) - (s_47_1));
        // D s_47_5: cast reint s_47_4 -> i64
        let s_47_5: i64 = (s_47_4 as i64);
        // D s_47_6: write-var gs#20211 <= s_47_5
        fn_state.gs_20211 = s_47_5;
        // D s_47_7: write-var i <= s_47_0
        fn_state.i = s_47_0;
        // N s_47_8: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var i:i64
        let s_48_0: i64 = fn_state.i;
        // D s_48_1: read-var gs#20211:i64
        let s_48_1: i64 = fn_state.gs_20211;
        // D s_48_2: cmp-gt s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) > (s_48_1));
        // N s_48_3: branch s_48_2 b53 b49
        if s_48_2 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // D s_49_1: create-sum enum = 0:"s_49_0"
        let s_49_1: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_49_0);
        // C s_49_2: const #8s : i
        let s_49_2: i128 = 8;
        // D s_49_3: read-var i:i64
        let s_49_3: i64 = fn_state.i;
        // D s_49_4: cast zx s_49_3 -> i
        let s_49_4: i128 = (i128::try_from(s_49_3).unwrap());
        // D s_49_5: mul s_49_2 s_49_4
        let s_49_5: i128 = ((s_49_2) * (s_49_4));
        // D s_49_6: cast reint s_49_5 -> i64
        let s_49_6: i64 = (s_49_5 as i64);
        // C s_49_7: const #8s : i
        let s_49_7: i128 = 8;
        // D s_49_8: cast zx s_49_6 -> i
        let s_49_8: i128 = (i128::try_from(s_49_6).unwrap());
        // D s_49_9: read-var value_name:bv
        let s_49_9: Bits = fn_state.value_name;
        // D s_49_10: bit-extract s_49_9 s_49_8 s_49_7
        let s_49_10: Bits = (Bits::new(
            ((s_49_9) >> (s_49_8)).value(),
            u16::try_from(s_49_7).unwrap(),
        ));
        // D s_49_11: cast reint s_49_10 -> u8
        let s_49_11: u8 = (s_49_10.value() as u8);
        // C s_49_12: const #1s : i
        let s_49_12: i128 = 1;
        // D s_49_13: cast zx s_49_11 -> bv
        let s_49_13: Bits = Bits::new(s_49_11 as u128, 8u16);
        // D s_49_14: read-var memaddrdesc:struct
        let s_49_14: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_49_15: read-var accdesc:struct
        let s_49_15: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_49_16: call PhysMemWrite(s_49_14, s_49_12, s_49_15, s_49_1, s_49_13)
        let s_49_16: ProductTypef8c3639b88223255 = PhysMemWrite(
            state,
            tracer,
            s_49_14,
            s_49_12,
            s_49_15,
            s_49_1,
            s_49_13,
        );
        // D s_49_17: write-var memstatus <= s_49_16
        fn_state.memstatus = s_49_16;
        // D s_49_18: read-var memstatus:struct
        let s_49_18: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_49_19: call IsFault__2(s_49_18)
        let s_49_19: bool = IsFault__2(state, tracer, s_49_18);
        // N s_49_20: branch s_49_19 b52 b50
        if s_49_19 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
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
        // D s_51_0: read-var memaddrdesc.3:struct
        let s_51_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_51_1: write-var ga#15686 <= s_51_0
        fn_state.ga_15686 = s_51_0;
        // D s_51_2: read-var ga#15686.0:struct
        let s_51_2: u64 = fn_state.ga_15686._0;
        // C s_51_3: const #1s : i
        let s_51_3: i128 = 1;
        // D s_51_4: cast zx s_51_2 -> bv
        let s_51_4: Bits = Bits::new(s_51_2 as u128, 56u16);
        // C s_51_5: cast cvt s_51_3 -> bv
        let s_51_5: Bits = Bits::new(s_51_3 as u128, 128);
        // D s_51_6: add s_51_4 s_51_5
        let s_51_6: Bits = (s_51_4 + s_51_5);
        // D s_51_7: cast reint s_51_6 -> u56
        let s_51_7: u64 = (s_51_6.value() as u64);
        // D s_51_8: write-var memaddrdesc.3.0 <= s_51_7
        fn_state.memaddrdesc._3._0 = s_51_7;
        // D s_51_9: read-var i:i64
        let s_51_9: i64 = fn_state.i;
        // C s_51_10: const #1s : i64
        let s_51_10: i64 = 1;
        // D s_51_11: add s_51_9 s_51_10
        let s_51_11: i64 = (s_51_9 + s_51_10);
        // D s_51_12: write-var i <= s_51_11
        fn_state.i = s_51_11;
        // N s_51_13: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1s : i
        let s_52_0: i128 = 1;
        // D s_52_1: read-var memstatus:struct
        let s_52_1: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_52_2: read-var memaddrdesc:struct
        let s_52_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_52_3: read-var accdesc:struct
        let s_52_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_52_4: call HandleExternalWriteAbort(s_52_1, s_52_2, s_52_0, s_52_3)
        let s_52_4: () = HandleExternalWriteAbort(
            state,
            tracer,
            s_52_1,
            s_52_2,
            s_52_0,
            s_52_3,
        );
        // N s_52_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #8s : i
        let s_54_0: i128 = 8;
        // D s_54_1: read-var size:i64
        let s_54_1: i64 = fn_state.size;
        // D s_54_2: cast zx s_54_1 -> i
        let s_54_2: i128 = (i128::try_from(s_54_1).unwrap());
        // D s_54_3: cmp-eq s_54_2 s_54_0
        let s_54_3: bool = ((s_54_2) == (s_54_0));
        // N s_54_4: branch s_54_3 b62 b55
        if s_54_3 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #16s : i
        let s_55_0: i128 = 16;
        // D s_55_1: read-var size:i64
        let s_55_1: i64 = fn_state.size;
        // D s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (i128::try_from(s_55_1).unwrap());
        // D s_55_3: cmp-eq s_55_2 s_55_0
        let s_55_3: bool = ((s_55_2) == (s_55_0));
        // D s_55_4: write-var gs#20224 <= s_55_3
        fn_state.gs_20224 = s_55_3;
        // N s_55_5: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#20224:u8
        let s_56_0: bool = fn_state.gs_20224;
        // N s_56_1: assert s_56_0
        let s_56_1: () = assert!(s_56_0);
        // C s_56_2: const #2s : i
        let s_56_2: i128 = 2;
        // D s_56_3: read-var size:i64
        let s_56_3: i64 = fn_state.size;
        // D s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // D s_56_5: div s_56_4 s_56_2
        let s_56_5: i128 = ((s_56_4) / (s_56_2));
        // D s_56_6: cast reint s_56_5 -> i64
        let s_56_6: i64 = (s_56_5 as i64);
        // D s_56_7: write-var halfsize <= s_56_6
        fn_state.halfsize = s_56_6;
        // D s_56_8: read-var value_name:bv
        let s_56_8: Bits = fn_state.value_name;
        // D s_56_9: size-of s_56_8
        let s_56_9: u16 = s_56_8.length();
        // D s_56_10: cast zx s_56_9 -> i
        let s_56_10: i128 = (i128::try_from(s_56_9).unwrap());
        // D s_56_11: cast reint s_56_10 -> i64
        let s_56_11: i64 = (s_56_10 as i64);
        // C s_56_12: const #1s : i
        let s_56_12: i128 = 1;
        // D s_56_13: cast zx s_56_11 -> i
        let s_56_13: i128 = (i128::try_from(s_56_11).unwrap());
        // D s_56_14: sub s_56_13 s_56_12
        let s_56_14: i128 = ((s_56_13) - (s_56_12));
        // D s_56_15: cast reint s_56_14 -> i64
        let s_56_15: i64 = (s_56_14 as i64);
        // D s_56_16: read-var value_name:bv
        let s_56_16: Bits = fn_state.value_name;
        // D s_56_17: size-of s_56_16
        let s_56_17: u16 = s_56_16.length();
        // D s_56_18: cast zx s_56_17 -> i
        let s_56_18: i128 = (i128::try_from(s_56_17).unwrap());
        // D s_56_19: cast reint s_56_18 -> i64
        let s_56_19: i64 = (s_56_18 as i64);
        // C s_56_20: const #1s : i
        let s_56_20: i128 = 1;
        // D s_56_21: cast zx s_56_19 -> i
        let s_56_21: i128 = (i128::try_from(s_56_19).unwrap());
        // D s_56_22: sub s_56_21 s_56_20
        let s_56_22: i128 = ((s_56_21) - (s_56_20));
        // D s_56_23: cast reint s_56_22 -> i64
        let s_56_23: i64 = (s_56_22 as i64);
        // D s_56_24: read-var halfsize:i64
        let s_56_24: i64 = fn_state.halfsize;
        // D s_56_25: cast zx s_56_24 -> i
        let s_56_25: i128 = (i128::try_from(s_56_24).unwrap());
        // D s_56_26: call __id(s_56_25)
        let s_56_26: i128 = u__id(state, tracer, s_56_25);
        // D s_56_27: cast reint s_56_26 -> i64
        let s_56_27: i64 = (s_56_26 as i64);
        // C s_56_28: const #8s : i
        let s_56_28: i128 = 8;
        // D s_56_29: cast zx s_56_27 -> i
        let s_56_29: i128 = (i128::try_from(s_56_27).unwrap());
        // D s_56_30: mul s_56_29 s_56_28
        let s_56_30: i128 = ((s_56_29) * (s_56_28));
        // D s_56_31: cast reint s_56_30 -> i64
        let s_56_31: i64 = (s_56_30 as i64);
        // D s_56_32: cast zx s_56_23 -> i
        let s_56_32: i128 = (i128::try_from(s_56_23).unwrap());
        // D s_56_33: cast zx s_56_31 -> i
        let s_56_33: i128 = (i128::try_from(s_56_31).unwrap());
        // D s_56_34: sub s_56_32 s_56_33
        let s_56_34: i128 = ((s_56_32) - (s_56_33));
        // D s_56_35: cast reint s_56_34 -> i64
        let s_56_35: i64 = (s_56_34 as i64);
        // C s_56_36: const #1s : i
        let s_56_36: i128 = 1;
        // D s_56_37: cast zx s_56_35 -> i
        let s_56_37: i128 = (i128::try_from(s_56_35).unwrap());
        // D s_56_38: add s_56_37 s_56_36
        let s_56_38: i128 = (s_56_37 + s_56_36);
        // D s_56_39: cast reint s_56_38 -> i64
        let s_56_39: i64 = (s_56_38 as i64);
        // D s_56_40: cast zx s_56_15 -> i
        let s_56_40: i128 = (i128::try_from(s_56_15).unwrap());
        // D s_56_41: cast zx s_56_39 -> i
        let s_56_41: i128 = (i128::try_from(s_56_39).unwrap());
        // D s_56_42: read-var value_name:bv
        let s_56_42: Bits = fn_state.value_name;
        // C s_56_43: const #1s : i64
        let s_56_43: i64 = 1;
        // C s_56_44: cast zx s_56_43 -> i
        let s_56_44: i128 = (i128::try_from(s_56_43).unwrap());
        // D s_56_45: sub s_56_40 s_56_41
        let s_56_45: i128 = ((s_56_40) - (s_56_41));
        // D s_56_46: add s_56_45 s_56_44
        let s_56_46: i128 = (s_56_45 + s_56_44);
        // D s_56_47: bit-extract s_56_42 s_56_41 s_56_46
        let s_56_47: Bits = (Bits::new(
            ((s_56_42) >> (s_56_41)).value(),
            u16::try_from(s_56_46).unwrap(),
        ));
        // D s_56_48: write-var highhalf <= s_56_47
        fn_state.highhalf = s_56_47;
        // D s_56_49: read-var value_name:bv
        let s_56_49: Bits = fn_state.value_name;
        // D s_56_50: size-of s_56_49
        let s_56_50: u16 = s_56_49.length();
        // D s_56_51: cast zx s_56_50 -> i
        let s_56_51: i128 = (i128::try_from(s_56_50).unwrap());
        // D s_56_52: cast reint s_56_51 -> i64
        let s_56_52: i64 = (s_56_51 as i64);
        // C s_56_53: const #1s : i
        let s_56_53: i128 = 1;
        // D s_56_54: cast zx s_56_52 -> i
        let s_56_54: i128 = (i128::try_from(s_56_52).unwrap());
        // D s_56_55: sub s_56_54 s_56_53
        let s_56_55: i128 = ((s_56_54) - (s_56_53));
        // D s_56_56: cast reint s_56_55 -> i64
        let s_56_56: i64 = (s_56_55 as i64);
        // D s_56_57: read-var halfsize:i64
        let s_56_57: i64 = fn_state.halfsize;
        // D s_56_58: cast zx s_56_57 -> i
        let s_56_58: i128 = (i128::try_from(s_56_57).unwrap());
        // D s_56_59: call __id(s_56_58)
        let s_56_59: i128 = u__id(state, tracer, s_56_58);
        // D s_56_60: cast reint s_56_59 -> i64
        let s_56_60: i64 = (s_56_59 as i64);
        // C s_56_61: const #8s : i
        let s_56_61: i128 = 8;
        // D s_56_62: cast zx s_56_60 -> i
        let s_56_62: i128 = (i128::try_from(s_56_60).unwrap());
        // D s_56_63: mul s_56_62 s_56_61
        let s_56_63: i128 = ((s_56_62) * (s_56_61));
        // D s_56_64: cast reint s_56_63 -> i64
        let s_56_64: i64 = (s_56_63 as i64);
        // D s_56_65: cast zx s_56_56 -> i
        let s_56_65: i128 = (i128::try_from(s_56_56).unwrap());
        // D s_56_66: cast zx s_56_64 -> i
        let s_56_66: i128 = (i128::try_from(s_56_64).unwrap());
        // D s_56_67: sub s_56_65 s_56_66
        let s_56_67: i128 = ((s_56_65) - (s_56_66));
        // D s_56_68: cast reint s_56_67 -> i64
        let s_56_68: i64 = (s_56_67 as i64);
        // D s_56_69: read-var value_name:bv
        let s_56_69: Bits = fn_state.value_name;
        // D s_56_70: size-of s_56_69
        let s_56_70: u16 = s_56_69.length();
        // D s_56_71: cast zx s_56_70 -> i
        let s_56_71: i128 = (i128::try_from(s_56_70).unwrap());
        // D s_56_72: cast reint s_56_71 -> i64
        let s_56_72: i64 = (s_56_71 as i64);
        // C s_56_73: const #1s : i
        let s_56_73: i128 = 1;
        // D s_56_74: cast zx s_56_72 -> i
        let s_56_74: i128 = (i128::try_from(s_56_72).unwrap());
        // D s_56_75: sub s_56_74 s_56_73
        let s_56_75: i128 = ((s_56_74) - (s_56_73));
        // D s_56_76: cast reint s_56_75 -> i64
        let s_56_76: i64 = (s_56_75 as i64);
        // D s_56_77: read-var halfsize:i64
        let s_56_77: i64 = fn_state.halfsize;
        // D s_56_78: cast zx s_56_77 -> i
        let s_56_78: i128 = (i128::try_from(s_56_77).unwrap());
        // D s_56_79: call __id(s_56_78)
        let s_56_79: i128 = u__id(state, tracer, s_56_78);
        // D s_56_80: cast reint s_56_79 -> i64
        let s_56_80: i64 = (s_56_79 as i64);
        // C s_56_81: const #8s : i
        let s_56_81: i128 = 8;
        // D s_56_82: cast zx s_56_80 -> i
        let s_56_82: i128 = (i128::try_from(s_56_80).unwrap());
        // D s_56_83: mul s_56_82 s_56_81
        let s_56_83: i128 = ((s_56_82) * (s_56_81));
        // D s_56_84: cast reint s_56_83 -> i64
        let s_56_84: i64 = (s_56_83 as i64);
        // D s_56_85: cast zx s_56_76 -> i
        let s_56_85: i128 = (i128::try_from(s_56_76).unwrap());
        // D s_56_86: cast zx s_56_84 -> i
        let s_56_86: i128 = (i128::try_from(s_56_84).unwrap());
        // D s_56_87: sub s_56_85 s_56_86
        let s_56_87: i128 = ((s_56_85) - (s_56_86));
        // D s_56_88: cast reint s_56_87 -> i64
        let s_56_88: i64 = (s_56_87 as i64);
        // D s_56_89: read-var halfsize:i64
        let s_56_89: i64 = fn_state.halfsize;
        // D s_56_90: cast zx s_56_89 -> i
        let s_56_90: i128 = (i128::try_from(s_56_89).unwrap());
        // D s_56_91: call __id(s_56_90)
        let s_56_91: i128 = u__id(state, tracer, s_56_90);
        // D s_56_92: cast reint s_56_91 -> i64
        let s_56_92: i64 = (s_56_91 as i64);
        // C s_56_93: const #8s : i
        let s_56_93: i128 = 8;
        // D s_56_94: cast zx s_56_92 -> i
        let s_56_94: i128 = (i128::try_from(s_56_92).unwrap());
        // D s_56_95: mul s_56_94 s_56_93
        let s_56_95: i128 = ((s_56_94) * (s_56_93));
        // D s_56_96: cast reint s_56_95 -> i64
        let s_56_96: i64 = (s_56_95 as i64);
        // D s_56_97: cast zx s_56_88 -> i
        let s_56_97: i128 = (i128::try_from(s_56_88).unwrap());
        // D s_56_98: cast zx s_56_96 -> i
        let s_56_98: i128 = (i128::try_from(s_56_96).unwrap());
        // D s_56_99: sub s_56_97 s_56_98
        let s_56_99: i128 = ((s_56_97) - (s_56_98));
        // D s_56_100: cast reint s_56_99 -> i64
        let s_56_100: i64 = (s_56_99 as i64);
        // C s_56_101: const #1s : i
        let s_56_101: i128 = 1;
        // D s_56_102: cast zx s_56_100 -> i
        let s_56_102: i128 = (i128::try_from(s_56_100).unwrap());
        // D s_56_103: add s_56_102 s_56_101
        let s_56_103: i128 = (s_56_102 + s_56_101);
        // D s_56_104: cast reint s_56_103 -> i64
        let s_56_104: i64 = (s_56_103 as i64);
        // D s_56_105: cast zx s_56_68 -> i
        let s_56_105: i128 = (i128::try_from(s_56_68).unwrap());
        // D s_56_106: cast zx s_56_104 -> i
        let s_56_106: i128 = (i128::try_from(s_56_104).unwrap());
        // D s_56_107: read-var value_name:bv
        let s_56_107: Bits = fn_state.value_name;
        // C s_56_108: const #1s : i64
        let s_56_108: i64 = 1;
        // C s_56_109: cast zx s_56_108 -> i
        let s_56_109: i128 = (i128::try_from(s_56_108).unwrap());
        // D s_56_110: sub s_56_105 s_56_106
        let s_56_110: i128 = ((s_56_105) - (s_56_106));
        // D s_56_111: add s_56_110 s_56_109
        let s_56_111: i128 = (s_56_110 + s_56_109);
        // D s_56_112: bit-extract s_56_107 s_56_106 s_56_111
        let s_56_112: Bits = (Bits::new(
            ((s_56_107) >> (s_56_106)).value(),
            u16::try_from(s_56_111).unwrap(),
        ));
        // C s_56_113: const #() : ()
        let s_56_113: () = ();
        // D s_56_114: create-sum enum = 0:"s_56_113"
        let s_56_114: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_56_113);
        // D s_56_115: read-var halfsize:i64
        let s_56_115: i64 = fn_state.halfsize;
        // D s_56_116: cast zx s_56_115 -> i
        let s_56_116: i128 = (i128::try_from(s_56_115).unwrap());
        // D s_56_117: read-var memaddrdesc:struct
        let s_56_117: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_56_118: read-var accdesc:struct
        let s_56_118: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_56_119: call PhysMemWrite(s_56_117, s_56_116, s_56_118, s_56_114, s_56_112)
        let s_56_119: ProductTypef8c3639b88223255 = PhysMemWrite(
            state,
            tracer,
            s_56_117,
            s_56_116,
            s_56_118,
            s_56_114,
            s_56_112,
        );
        // D s_56_120: write-var memstatus <= s_56_119
        fn_state.memstatus = s_56_119;
        // D s_56_121: read-var memstatus:struct
        let s_56_121: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_56_122: call IsFault__2(s_56_121)
        let s_56_122: bool = IsFault__2(state, tracer, s_56_121);
        // N s_56_123: branch s_56_122 b61 b57
        if s_56_122 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var memaddrdesc.3:struct
        let s_58_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_58_1: write-var ga#15678 <= s_58_0
        fn_state.ga_15678 = s_58_0;
        // D s_58_2: read-var ga#15678.0:struct
        let s_58_2: u64 = fn_state.ga_15678._0;
        // D s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 56u16);
        // D s_58_4: read-var halfsize:i64
        let s_58_4: i64 = fn_state.halfsize;
        // D s_58_5: cast zx s_58_4 -> i
        let s_58_5: i128 = (i128::try_from(s_58_4).unwrap());
        // D s_58_6: cast cvt s_58_5 -> bv
        let s_58_6: Bits = Bits::new(s_58_5 as u128, 128);
        // D s_58_7: add s_58_3 s_58_6
        let s_58_7: Bits = (s_58_3 + s_58_6);
        // D s_58_8: cast reint s_58_7 -> u56
        let s_58_8: u64 = (s_58_7.value() as u64);
        // D s_58_9: write-var memaddrdesc.3.0 <= s_58_8
        fn_state.memaddrdesc._3._0 = s_58_8;
        // C s_58_10: const #() : ()
        let s_58_10: () = ();
        // D s_58_11: create-sum enum = 0:"s_58_10"
        let s_58_11: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_58_10);
        // D s_58_12: read-var halfsize:i64
        let s_58_12: i64 = fn_state.halfsize;
        // D s_58_13: cast zx s_58_12 -> i
        let s_58_13: i128 = (i128::try_from(s_58_12).unwrap());
        // D s_58_14: read-var memaddrdesc:struct
        let s_58_14: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_58_15: read-var accdesc:struct
        let s_58_15: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_58_16: read-var highhalf:bv
        let s_58_16: Bits = fn_state.highhalf;
        // D s_58_17: call PhysMemWrite(s_58_14, s_58_13, s_58_15, s_58_11, s_58_16)
        let s_58_17: ProductTypef8c3639b88223255 = PhysMemWrite(
            state,
            tracer,
            s_58_14,
            s_58_13,
            s_58_15,
            s_58_11,
            s_58_16,
        );
        // D s_58_18: write-var memstatusshadow#325 <= s_58_17
        fn_state.memstatusshadow_325 = s_58_17;
        // D s_58_19: read-var memstatusshadow#325:struct
        let s_58_19: ProductTypef8c3639b88223255 = fn_state.memstatusshadow_325;
        // D s_58_20: call IsFault__2(s_58_19)
        let s_58_20: bool = IsFault__2(state, tracer, s_58_19);
        // N s_58_21: branch s_58_20 b60 b59
        if s_58_20 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var halfsize:i64
        let s_60_0: i64 = fn_state.halfsize;
        // D s_60_1: cast zx s_60_0 -> i
        let s_60_1: i128 = (i128::try_from(s_60_0).unwrap());
        // D s_60_2: read-var memstatusshadow#325:struct
        let s_60_2: ProductTypef8c3639b88223255 = fn_state.memstatusshadow_325;
        // D s_60_3: read-var memaddrdesc:struct
        let s_60_3: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_60_4: read-var accdesc:struct
        let s_60_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_60_5: call HandleExternalWriteAbort(s_60_2, s_60_3, s_60_1, s_60_4)
        let s_60_5: () = HandleExternalWriteAbort(
            state,
            tracer,
            s_60_2,
            s_60_3,
            s_60_1,
            s_60_4,
        );
        // N s_60_6: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var halfsize:i64
        let s_61_0: i64 = fn_state.halfsize;
        // D s_61_1: cast zx s_61_0 -> i
        let s_61_1: i128 = (i128::try_from(s_61_0).unwrap());
        // D s_61_2: read-var memstatus:struct
        let s_61_2: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_61_3: read-var memaddrdesc:struct
        let s_61_3: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_61_4: read-var accdesc:struct
        let s_61_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_61_5: call HandleExternalWriteAbort(s_61_2, s_61_3, s_61_1, s_61_4)
        let s_61_5: () = HandleExternalWriteAbort(
            state,
            tracer,
            s_61_2,
            s_61_3,
            s_61_1,
            s_61_4,
        );
        // N s_61_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#20224 <= s_62_0
        fn_state.gs_20224 = s_62_0;
        // N s_62_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var ispair:u8
        let s_63_0: bool = fn_state.ispair;
        // D s_63_1: write-var gs#20207 <= s_63_0
        fn_state.gs_20207 = s_63_0;
        // N s_63_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // D s_64_1: create-sum enum = 0:"s_64_0"
        let s_64_1: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_64_0);
        // D s_64_2: read-var size:i64
        let s_64_2: i64 = fn_state.size;
        // D s_64_3: cast zx s_64_2 -> i
        let s_64_3: i128 = (i128::try_from(s_64_2).unwrap());
        // D s_64_4: read-var memaddrdesc:struct
        let s_64_4: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_64_5: read-var accdesc:struct
        let s_64_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_64_6: read-var value_name:bv
        let s_64_6: Bits = fn_state.value_name;
        // D s_64_7: call PhysMemWrite(s_64_4, s_64_3, s_64_5, s_64_1, s_64_6)
        let s_64_7: ProductTypef8c3639b88223255 = PhysMemWrite(
            state,
            tracer,
            s_64_4,
            s_64_3,
            s_64_5,
            s_64_1,
            s_64_6,
        );
        // D s_64_8: write-var memstatusshadow#326 <= s_64_7
        fn_state.memstatusshadow_326 = s_64_7;
        // D s_64_9: read-var memstatusshadow#326:struct
        let s_64_9: ProductTypef8c3639b88223255 = fn_state.memstatusshadow_326;
        // D s_64_10: call IsFault__2(s_64_9)
        let s_64_10: bool = IsFault__2(state, tracer, s_64_9);
        // N s_64_11: branch s_64_10 b66 b65
        if s_64_10 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var size:i64
        let s_66_0: i64 = fn_state.size;
        // D s_66_1: cast zx s_66_0 -> i
        let s_66_1: i128 = (i128::try_from(s_66_0).unwrap());
        // D s_66_2: read-var memstatusshadow#326:struct
        let s_66_2: ProductTypef8c3639b88223255 = fn_state.memstatusshadow_326;
        // D s_66_3: read-var memaddrdesc:struct
        let s_66_3: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_66_4: read-var accdesc:struct
        let s_66_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_66_5: call HandleExternalWriteAbort(s_66_2, s_66_3, s_66_1, s_66_4)
        let s_66_5: () = HandleExternalWriteAbort(
            state,
            tracer,
            s_66_2,
            s_66_3,
            s_66_1,
            s_66_4,
        );
        // N s_66_6: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var ispair:u8
        let s_67_0: bool = fn_state.ispair;
        // D s_67_1: not s_67_0
        let s_67_1: bool = !s_67_0;
        // D s_67_2: write-var atomic <= s_67_1
        fn_state.atomic = s_67_1;
        // N s_67_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var aligned:u8
        let s_68_0: bool = fn_state.aligned;
        // D s_68_1: not s_68_0
        let s_68_1: bool = !s_68_0;
        // N s_68_2: branch s_68_1 b73 b69
        if s_68_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#20251 <= s_69_0
        fn_state.gs_20251 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#20251:u8
        let s_70_0: bool = fn_state.gs_20251;
        // N s_70_1: branch s_70_0 b72 b71
        if s_70_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var atomic <= s_71_0
        fn_state.atomic = s_71_0;
        // N s_71_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var accdesc:struct
        let s_72_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_72_1: call AlignmentFault(s_72_0)
        let s_72_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_72_0);
        // D s_72_2: read-var address:u64
        let s_72_2: u64 = fn_state.address;
        // D s_72_3: call AArch64_Abort(s_72_2, s_72_1)
        let s_72_3: () = AArch64_Abort(state, tracer, s_72_2, s_72_1);
        // N s_72_4: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #67u : u32
        let s_73_0: u32 = 67;
        // S s_73_1: call ConstrainUnpredictableBool(s_73_0)
        let s_73_1: bool = ConstrainUnpredictableBool(state, tracer, s_73_0);
        // S s_73_2: not s_73_1
        let s_73_2: bool = !s_73_1;
        // D s_73_3: write-var gs#20251 <= s_73_2
        fn_state.gs_20251 = s_73_2;
        // N s_73_4: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#20250 <= s_74_0
        fn_state.gs_20250 = s_74_0;
        // N s_74_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#20249 <= s_75_0
        fn_state.gs_20249 = s_75_0;
        // N s_75_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#20248 <= s_76_0
        fn_state.gs_20248 = s_76_0;
        // N s_76_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // D s_77_1: write-var gs#20247 <= s_77_0
        fn_state.gs_20247 = s_77_0;
        // N s_77_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #1u : u8
        let s_78_0: bool = true;
        // D s_78_1: write-var atomic <= s_78_0
        fn_state.atomic = s_78_0;
        // N s_78_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var memaddrdesc.2:struct
        let s_79_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_79_1: write-var ga#15634 <= s_79_0
        fn_state.ga_15634 = s_79_0;
        // D s_79_2: read-var ga#15634.4:struct
        let s_79_2: ProductType183e6678e5239c85 = fn_state.ga_15634._4;
        // D s_79_3: write-var ga#15635 <= s_79_2
        fn_state.ga_15635 = s_79_2;
        // D s_79_4: read-var ga#15635.0:struct
        let s_79_4: u8 = fn_state.ga_15635._0;
        // D s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 2u16);
        // C s_79_6: const #480u : u32
        let s_79_6: u32 = 480;
        // D s_79_7: read-reg s_79_6:u8
        let s_79_7: u8 = {
            let value = state.read_register::<u8>(s_79_6 as isize);
            tracer.read_register(s_79_6 as isize, value);
            value
        };
        // D s_79_8: cast zx s_79_7 -> bv
        let s_79_8: Bits = Bits::new(s_79_7 as u128, 2u16);
        // D s_79_9: cmp-eq s_79_5 s_79_8
        let s_79_9: bool = ((s_79_5) == (s_79_8));
        // D s_79_10: write-var gs#20205 <= s_79_9
        fn_state.gs_20205 = s_79_9;
        // N s_79_11: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var memaddrdesc.2:struct
        let s_80_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_80_1: write-var ga#15630 <= s_80_0
        fn_state.ga_15630 = s_80_0;
        // D s_80_2: read-var ga#15630.1:struct
        let s_80_2: ProductType183e6678e5239c85 = fn_state.ga_15630._1;
        // D s_80_3: write-var ga#15631 <= s_80_2
        fn_state.ga_15631 = s_80_2;
        // D s_80_4: read-var ga#15631.0:struct
        let s_80_4: u8 = fn_state.ga_15631._0;
        // D s_80_5: cast zx s_80_4 -> bv
        let s_80_5: Bits = Bits::new(s_80_4 as u128, 2u16);
        // C s_80_6: const #480u : u32
        let s_80_6: u32 = 480;
        // D s_80_7: read-reg s_80_6:u8
        let s_80_7: u8 = {
            let value = state.read_register::<u8>(s_80_6 as isize);
            tracer.read_register(s_80_6 as isize, value);
            value
        };
        // D s_80_8: cast zx s_80_7 -> bv
        let s_80_8: Bits = Bits::new(s_80_7 as u128, 2u16);
        // D s_80_9: cmp-eq s_80_5 s_80_8
        let s_80_9: bool = ((s_80_5) == (s_80_8));
        // D s_80_10: write-var gs#20204 <= s_80_9
        fn_state.gs_20204 = s_80_9;
        // N s_80_11: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: read-var accdesc:struct
        let s_81_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_81_2: read-var memaddrdesc:struct
        let s_81_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_81_3: call SPESampleLoadStore(s_81_0, s_81_1, s_81_2)
        let s_81_3: () = SPESampleLoadStore(state, tracer, s_81_0, s_81_1, s_81_2);
        // N s_81_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var address:u64
        let s_82_0: u64 = fn_state.address;
        // D s_82_1: call AArch64_PhysicalTag(s_82_0)
        let s_82_1: u8 = AArch64_PhysicalTag(state, tracer, s_82_0);
        // D s_82_2: read-var memaddrdesc:struct
        let s_82_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_82_3: read-var accdesc:struct
        let s_82_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_82_4: call AArch64_CheckTag(s_82_2, s_82_3, s_82_1)
        let s_82_4: bool = AArch64_CheckTag(state, tracer, s_82_2, s_82_3, s_82_1);
        // D s_82_5: write-var ga#15626 <= s_82_4
        fn_state.ga_15626 = s_82_4;
        // N s_82_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var ga#15626:u8
        let s_83_0: bool = fn_state.ga_15626;
        // D s_83_1: not s_83_0
        let s_83_1: bool = !s_83_0;
        // N s_83_2: branch s_83_1 b86 b84
        if s_83_1 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_85_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var address:u64
        let s_86_0: u64 = fn_state.address;
        // D s_86_1: read-var accdesc:struct
        let s_86_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_86_2: call AArch64_TagCheckFault(s_86_0, s_86_1)
        let s_86_2: () = AArch64_TagCheckFault(state, tracer, s_86_0, s_86_1);
        // N s_86_3: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var accdesc.28:struct
        let s_87_0: bool = fn_state.accdesc._28;
        // D s_87_1: write-var gs#20195 <= s_87_0
        fn_state.gs_20195 = s_87_0;
        // N s_87_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var accdesc.30:struct
        let s_88_0: bool = fn_state.accdesc._30;
        // N s_88_1: branch s_88_0 b94 b89
        if s_88_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#20200 <= s_89_0
        fn_state.gs_20200 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#20200:u8
        let s_90_0: bool = fn_state.gs_20200;
        // N s_90_1: branch s_90_0 b93 b91
        if s_90_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_91_0: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_92_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #7u : u32
        let s_93_0: u32 = 7;
        // C s_93_1: const #0u : u8
        let s_93_1: bool = false;
        // S s_93_2: call FailTransaction(s_93_0, s_93_1)
        let s_93_2: () = FailTransaction(state, tracer, s_93_0, s_93_1);
        // N s_93_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var memaddrdesc.2:struct
        let s_94_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_94_1: call MemHasTransactionalAccess(s_94_0)
        let s_94_1: bool = MemHasTransactionalAccess(state, tracer, s_94_0);
        // D s_94_2: not s_94_1
        let s_94_2: bool = !s_94_1;
        // D s_94_3: write-var gs#20200 <= s_94_2
        fn_state.gs_20200 = s_94_2;
        // N s_94_4: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var memaddrdesc.3:struct
        let s_95_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_95_1: const #() : ()
        let s_95_1: () = ();
        // S s_95_2: call ProcessorID(s_95_1)
        let s_95_2: i128 = ProcessorID(state, tracer, s_95_1);
        // D s_95_3: read-var size:i64
        let s_95_3: i64 = fn_state.size;
        // D s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // D s_95_5: call ClearExclusiveByAddress(s_95_0, s_95_2, s_95_4)
        let s_95_5: () = ClearExclusiveByAddress(state, tracer, s_95_0, s_95_2, s_95_4);
        // N s_95_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var memaddrdesc.0:struct
        let s_96_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_96_1: read-var address:u64
        let s_96_1: u64 = fn_state.address;
        // D s_96_2: call AArch64_Abort(s_96_1, s_96_0)
        let s_96_2: () = AArch64_Abort(state, tracer, s_96_1, s_96_0);
        // N s_96_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var address:u64
        let s_97_0: u64 = fn_state.address;
        // D s_97_1: read-var accdesc:struct
        let s_97_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_97_2: call AArch64_AccessIsTagChecked(s_97_0, s_97_1)
        let s_97_2: bool = AArch64_AccessIsTagChecked(state, tracer, s_97_0, s_97_1);
        // D s_97_3: write-var accdesc.28 <= s_97_2
        fn_state.accdesc._28 = s_97_2;
        // N s_97_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var accdesc.28:struct
        let s_98_0: bool = fn_state.accdesc._28;
        // D s_98_1: write-var gs#20191 <= s_98_0
        fn_state.gs_20191 = s_98_0;
        // N s_98_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #16s : i
        let s_99_0: i128 = 16;
        // D s_99_1: read-var size:i64
        let s_99_1: i64 = fn_state.size;
        // D s_99_2: cast zx s_99_1 -> i
        let s_99_2: i128 = (i128::try_from(s_99_1).unwrap());
        // D s_99_3: read-var address:u64
        let s_99_3: u64 = fn_state.address;
        // D s_99_4: call AllInAlignedQuantity(s_99_3, s_99_2, s_99_0)
        let s_99_4: bool = AllInAlignedQuantity(state, tracer, s_99_3, s_99_2, s_99_0);
        // N s_99_5: assert s_99_4
        let s_99_5: () = assert!(s_99_4);
        // N s_99_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // D s_100_1: write-var gs#20186 <= s_100_0
        fn_state.gs_20186 = s_100_0;
        // N s_100_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #1u : u8
        let s_101_0: bool = true;
        // D s_101_1: write-var gs#20187 <= s_101_0
        fn_state.gs_20187 = s_101_0;
        // N s_101_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #1u : u8
        let s_102_0: bool = true;
        // D s_102_1: write-var gs#20188 <= s_102_0
        fn_state.gs_20188 = s_102_0;
        // N s_102_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #1u : u8
        let s_103_0: bool = true;
        // D s_103_1: write-var gs#20189 <= s_103_0
        fn_state.gs_20189 = s_103_0;
        // N s_103_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
