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
use HandleExternalReadAbort::*;
use IsAligned__1::*;
use u__IMPDEF_boolean::*;
use SPESampleLoadStore::*;
use AArch64_TagCheckFault::*;
use MemHasTransactionalAccess::*;
use AArch64_TranslateAddress::*;
use HaveTME::*;
use AArch64_AccessIsTagChecked::*;
use PhysMemRead::*;
use AArch64_Abort::*;
use IsFault::*;
use AlignmentFault::*;
use AArch64_CheckTag::*;
use common::*;
pub fn AArch64_MemSingle_read__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i64,
    accdesc_in: ProductType9878976b5bcce9c9,
    aligned: bool,
    ispair: bool,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_20126: bool,
        gs_20130: bool,
        gs_20114: bool,
        gs_20121: bool,
        gs_20138: bool,
        ga_15562: ProductType183e6678e5239c85,
        gs_20142: i64,
        ga_15600: ProductTypeda0231e9dc169f81,
        value_name: Bits,
        ga_15565: ProductTypef170cab34335b70c,
        gs_20117: bool,
        gs_20115: bool,
        gs_20112: bool,
        gs_20134: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        ga_15566: ProductType183e6678e5239c85,
        ga_15580: ProductType2b2aba4822138824,
        gs_20133: bool,
        gs_20113: bool,
        halfsize: i64,
        gs_20159: bool,
        gs_20136: bool,
        ga_15559: ProductTypef170cab34335b70c,
        ga_15587: ProductType2b2aba4822138824,
        ga_15561: ProductTypef170cab34335b70c,
        ga_15557: bool,
        highhalf: Bits,
        gs_20132: bool,
        memstatus: ProductTypef8c3639b88223255,
        accdesc: ProductType9878976b5bcce9c9,
        gs_20129: bool,
        ga_15592: ProductType2b2aba4822138824,
        i: i64,
        gs_446900: ProductType2b2aba4822138824,
        atomic: bool,
        lowhalf: Bits,
        gs_20135: bool,
        ga_15589: ProductTypeda0231e9dc169f81,
        address: u64,
        size: i64,
        accdesc_in: ProductType9878976b5bcce9c9,
        aligned: bool,
        ispair: bool,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc_in,
        aligned,
        ispair,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
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
    ) -> Bits {
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
    ) -> Bits {
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
    ) -> Bits {
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
    ) -> Bits {
        // C s_4_0: const #16s : i
        let s_4_0: i128 = 16;
        // D s_4_1: read-var size:i64
        let s_4_1: i64 = fn_state.size;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // D s_4_4: write-var gs#20112 <= s_4_3
        fn_state.gs_20112 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var gs#20112:u8
        let s_5_0: bool = fn_state.gs_20112;
        // D s_5_1: write-var gs#20113 <= s_5_0
        fn_state.gs_20113 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#20113:u8
        let s_6_0: bool = fn_state.gs_20113;
        // D s_6_1: write-var gs#20114 <= s_6_0
        fn_state.gs_20114 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#20114:u8
        let s_7_0: bool = fn_state.gs_20114;
        // D s_7_1: write-var gs#20115 <= s_7_0
        fn_state.gs_20115 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#20115:u8
        let s_8_0: bool = fn_state.gs_20115;
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
    ) -> Bits {
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
    ) -> Bits {
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
    ) -> Bits {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#20117 <= s_11_0
        fn_state.gs_20117 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_12_0: read-var gs#20117:u8
        let s_12_0: bool = fn_state.gs_20117;
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
    ) -> Bits {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
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
    ) -> Bits {
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
    ) -> Bits {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HaveTME(s_17_0)
        let s_17_1: bool = HaveTME(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b89 b18
        if s_17_1 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveMTE2Ext(s_19_0)
        let s_19_1: bool = HaveMTE2Ext(state, tracer, s_19_0);
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
    ) -> Bits {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#20121 <= s_20_0
        fn_state.gs_20121 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_21_0: read-var gs#20121:u8
        let s_21_0: bool = fn_state.gs_20121;
        // N s_21_1: branch s_21_0 b83 b22
        if s_21_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_23_0: const #22416u : u32
        let s_23_0: u32 = 22416;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: bool = {
            let value = state.read_register::<bool>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // N s_23_2: branch s_23_1 b82 b24
        if s_23_1 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_25_0: read-var memaddrdesc.2:struct
        let s_25_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_25_1: write-var ga#15559 <= s_25_0
        fn_state.ga_15559 = s_25_0;
        // D s_25_2: read-var ga#15559.2:struct
        let s_25_2: u32 = fn_state.ga_15559._2;
        // C s_25_3: const #0u : u32
        let s_25_3: u32 = 0;
        // D s_25_4: cmp-eq s_25_2 s_25_3
        let s_25_4: bool = ((s_25_2) == (s_25_3));
        // N s_25_5: branch s_25_4 b81 b26
        if s_25_4 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#20129 <= s_26_0
        fn_state.gs_20129 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_27_0: read-var gs#20129:u8
        let s_27_0: bool = fn_state.gs_20129;
        // N s_27_1: branch s_27_0 b80 b28
        if s_27_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#20130 <= s_28_0
        fn_state.gs_20130 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_29_0: read-var gs#20130:u8
        let s_29_0: bool = fn_state.gs_20130;
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
    ) -> Bits {
        // D s_30_0: read-var accdesc.9:struct
        let s_30_0: bool = fn_state.accdesc._9;
        // N s_30_1: branch s_30_0 b78 b31
        if s_30_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_31_0: read-var accdesc.4:struct
        let s_31_0: bool = fn_state.accdesc._4;
        // D s_31_1: write-var gs#20132 <= s_31_0
        fn_state.gs_20132 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_32_0: read-var gs#20132:u8
        let s_32_0: bool = fn_state.gs_20132;
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
    ) -> Bits {
        // D s_33_0: read-var accdesc.3:struct
        let s_33_0: bool = fn_state.accdesc._3;
        // D s_33_1: write-var gs#20133 <= s_33_0
        fn_state.gs_20133 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_34_0: read-var gs#20133:u8
        let s_34_0: bool = fn_state.gs_20133;
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
    ) -> Bits {
        // D s_35_0: read-var accdesc.2:struct
        let s_35_0: bool = fn_state.accdesc._2;
        // D s_35_1: write-var gs#20134 <= s_35_0
        fn_state.gs_20134 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_36_0: read-var gs#20134:u8
        let s_36_0: bool = fn_state.gs_20134;
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
    ) -> Bits {
        // D s_37_0: read-var accdesc.24:struct
        let s_37_0: bool = fn_state.accdesc._24;
        // D s_37_1: write-var gs#20135 <= s_37_0
        fn_state.gs_20135 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_38_0: read-var gs#20135:u8
        let s_38_0: bool = fn_state.gs_20135;
        // N s_38_1: branch s_38_0 b69 b39
        if s_38_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_39_0: read-var aligned:u8
        let s_39_0: bool = fn_state.aligned;
        // N s_39_1: branch s_39_0 b68 b40
        if s_39_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_40_0: const #"FEAT_LSE2: access is atomic" : str
        let s_40_0: &'static str = "FEAT_LSE2: access is atomic";
        // S s_40_1: call __IMPDEF_boolean(s_40_0)
        let s_40_1: bool = u__IMPDEF_boolean(state, tracer, s_40_0);
        // D s_40_2: write-var atomic <= s_40_1
        fn_state.atomic = s_40_1;
        // N s_40_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_41_0: read-var atomic:u8
        let s_41_0: bool = fn_state.atomic;
        // N s_41_1: branch s_41_0 b64 b42
        if s_41_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_42_0: read-var aligned:u8
        let s_42_0: bool = fn_state.aligned;
        // N s_42_1: branch s_42_0 b63 b43
        if s_42_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#20138 <= s_43_0
        fn_state.gs_20138 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_44_0: read-var gs#20138:u8
        let s_44_0: bool = fn_state.gs_20138;
        // N s_44_1: branch s_44_0 b53 b45
        if s_44_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_45_0: const #0s : i64
        let s_45_0: i64 = 0;
        // C s_45_1: const #1s : i
        let s_45_1: i128 = 1;
        // D s_45_2: read-var size:i64
        let s_45_2: i64 = fn_state.size;
        // D s_45_3: cast zx s_45_2 -> i
        let s_45_3: i128 = (i128::try_from(s_45_2).unwrap());
        // D s_45_4: sub s_45_3 s_45_1
        let s_45_4: i128 = ((s_45_3) - (s_45_1));
        // D s_45_5: cast reint s_45_4 -> i64
        let s_45_5: i64 = (s_45_4 as i64);
        // D s_45_6: write-var gs#20142 <= s_45_5
        fn_state.gs_20142 = s_45_5;
        // D s_45_7: write-var i <= s_45_0
        fn_state.i = s_45_0;
        // N s_45_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_46_0: read-var i:i64
        let s_46_0: i64 = fn_state.i;
        // D s_46_1: read-var gs#20142:i64
        let s_46_1: i64 = fn_state.gs_20142;
        // D s_46_2: cmp-gt s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) > (s_46_1));
        // N s_46_3: branch s_46_2 b51 b47
        if s_46_2 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // D s_47_1: create-sum enum = 0:"s_47_0"
        let s_47_1: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_47_0);
        // C s_47_2: const #1s : i
        let s_47_2: i128 = 1;
        // D s_47_3: read-var memaddrdesc:struct
        let s_47_3: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_47_4: read-var accdesc:struct
        let s_47_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_47_5: call PhysMemRead(s_47_3, s_47_2, s_47_4, s_47_1)
        let s_47_5: ProductType2b2aba4822138824 = PhysMemRead(
            state,
            tracer,
            s_47_3,
            s_47_2,
            s_47_4,
            s_47_1,
        );
        // D s_47_6: write-var gs#446900 <= s_47_5
        fn_state.gs_446900 = s_47_5;
        // D s_47_7: read-var gs#446900.0:struct
        let s_47_7: ProductTypef8c3639b88223255 = fn_state.gs_446900._0;
        // D s_47_8: read-var gs#446900.1:struct
        let s_47_8: Bits = fn_state.gs_446900._1;
        // D s_47_9: cast reint s_47_8 -> u8
        let s_47_9: u8 = (s_47_8.value() as u8);
        // D s_47_10: write-var memstatus <= s_47_7
        fn_state.memstatus = s_47_7;
        // C s_47_11: const #8s : i
        let s_47_11: i128 = 8;
        // D s_47_12: read-var i:i64
        let s_47_12: i64 = fn_state.i;
        // D s_47_13: cast zx s_47_12 -> i
        let s_47_13: i128 = (i128::try_from(s_47_12).unwrap());
        // D s_47_14: mul s_47_11 s_47_13
        let s_47_14: i128 = ((s_47_11) * (s_47_13));
        // D s_47_15: cast reint s_47_14 -> i64
        let s_47_15: i64 = (s_47_14 as i64);
        // C s_47_16: const #7s : i
        let s_47_16: i128 = 7;
        // D s_47_17: cast zx s_47_15 -> i
        let s_47_17: i128 = (i128::try_from(s_47_15).unwrap());
        // D s_47_18: add s_47_17 s_47_16
        let s_47_18: i128 = (s_47_17 + s_47_16);
        // D s_47_19: cast reint s_47_18 -> i64
        let s_47_19: i64 = (s_47_18 as i64);
        // C s_47_20: const #8s : i
        let s_47_20: i128 = 8;
        // D s_47_21: read-var i:i64
        let s_47_21: i64 = fn_state.i;
        // D s_47_22: cast zx s_47_21 -> i
        let s_47_22: i128 = (i128::try_from(s_47_21).unwrap());
        // D s_47_23: mul s_47_20 s_47_22
        let s_47_23: i128 = ((s_47_20) * (s_47_22));
        // D s_47_24: cast reint s_47_23 -> i64
        let s_47_24: i64 = (s_47_23 as i64);
        // D s_47_25: cast zx s_47_19 -> i
        let s_47_25: i128 = (i128::try_from(s_47_19).unwrap());
        // D s_47_26: cast zx s_47_24 -> i
        let s_47_26: i128 = (i128::try_from(s_47_24).unwrap());
        // D s_47_27: cast zx s_47_9 -> bv
        let s_47_27: Bits = Bits::new(s_47_9 as u128, 8u16);
        // D s_47_28: read-var value_name:bv
        let s_47_28: Bits = fn_state.value_name;
        // D s_47_29: sub s_47_25 s_47_26
        let s_47_29: i128 = ((s_47_25) - (s_47_26));
        // C s_47_30: const #1u : u64
        let s_47_30: u64 = 1;
        // C s_47_31: cast zx s_47_30 -> bv
        let s_47_31: Bits = Bits::new(s_47_30 as u128, 64u16);
        // D s_47_32: lsl s_47_31 s_47_29
        let s_47_32: Bits = s_47_31 << s_47_29;
        // D s_47_33: sub s_47_32 s_47_31
        let s_47_33: Bits = ((s_47_32) - (s_47_31));
        // D s_47_34: and s_47_27 s_47_33
        let s_47_34: Bits = ((s_47_27) & (s_47_33));
        // D s_47_35: lsl s_47_34 s_47_26
        let s_47_35: Bits = s_47_34 << s_47_26;
        // D s_47_36: lsl s_47_33 s_47_26
        let s_47_36: Bits = s_47_33 << s_47_26;
        // D s_47_37: cmpl s_47_36
        let s_47_37: Bits = !s_47_36;
        // D s_47_38: and s_47_28 s_47_37
        let s_47_38: Bits = ((s_47_28) & (s_47_37));
        // D s_47_39: or s_47_38 s_47_35
        let s_47_39: Bits = ((s_47_38) | (s_47_35));
        // D s_47_40: write-var value_name <= s_47_39
        fn_state.value_name = s_47_39;
        // D s_47_41: read-var memstatus:struct
        let s_47_41: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_47_42: call IsFault__2(s_47_41)
        let s_47_42: bool = IsFault__2(state, tracer, s_47_41);
        // N s_47_43: branch s_47_42 b50 b48
        if s_47_42 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_49_0: read-var memaddrdesc.3:struct
        let s_49_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_49_1: write-var ga#15600 <= s_49_0
        fn_state.ga_15600 = s_49_0;
        // D s_49_2: read-var ga#15600.0:struct
        let s_49_2: u64 = fn_state.ga_15600._0;
        // C s_49_3: const #1s : i
        let s_49_3: i128 = 1;
        // D s_49_4: cast zx s_49_2 -> bv
        let s_49_4: Bits = Bits::new(s_49_2 as u128, 56u16);
        // C s_49_5: cast cvt s_49_3 -> bv
        let s_49_5: Bits = Bits::new(s_49_3 as u128, 128);
        // D s_49_6: add s_49_4 s_49_5
        let s_49_6: Bits = (s_49_4 + s_49_5);
        // D s_49_7: cast reint s_49_6 -> u56
        let s_49_7: u64 = (s_49_6.value() as u64);
        // D s_49_8: write-var memaddrdesc.3.0 <= s_49_7
        fn_state.memaddrdesc._3._0 = s_49_7;
        // D s_49_9: read-var i:i64
        let s_49_9: i64 = fn_state.i;
        // C s_49_10: const #1s : i64
        let s_49_10: i64 = 1;
        // D s_49_11: add s_49_9 s_49_10
        let s_49_11: i64 = (s_49_9 + s_49_10);
        // D s_49_12: write-var i <= s_49_11
        fn_state.i = s_49_11;
        // N s_49_13: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_50_0: const #1s : i
        let s_50_0: i128 = 1;
        // D s_50_1: read-var memstatus:struct
        let s_50_1: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_50_2: read-var memaddrdesc:struct
        let s_50_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_50_3: read-var accdesc:struct
        let s_50_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_50_4: call HandleExternalReadAbort(s_50_1, s_50_2, s_50_0, s_50_3)
        let s_50_4: () = HandleExternalReadAbort(
            state,
            tracer,
            s_50_1,
            s_50_2,
            s_50_0,
            s_50_3,
        );
        // N s_50_5: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_52_0: read-var value_name:bv
        let s_52_0: Bits = fn_state.value_name;
        // N s_52_1: return s_52_0
        return s_52_0;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_53_0: const #8s : i
        let s_53_0: i128 = 8;
        // D s_53_1: read-var size:i64
        let s_53_1: i64 = fn_state.size;
        // D s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (i128::try_from(s_53_1).unwrap());
        // D s_53_3: cmp-eq s_53_2 s_53_0
        let s_53_3: bool = ((s_53_2) == (s_53_0));
        // N s_53_4: branch s_53_3 b62 b54
        if s_53_3 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_54_0: const #16s : i
        let s_54_0: i128 = 16;
        // D s_54_1: read-var size:i64
        let s_54_1: i64 = fn_state.size;
        // D s_54_2: cast zx s_54_1 -> i
        let s_54_2: i128 = (i128::try_from(s_54_1).unwrap());
        // D s_54_3: cmp-eq s_54_2 s_54_0
        let s_54_3: bool = ((s_54_2) == (s_54_0));
        // D s_54_4: write-var gs#20159 <= s_54_3
        fn_state.gs_20159 = s_54_3;
        // N s_54_5: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_55_0: read-var gs#20159:u8
        let s_55_0: bool = fn_state.gs_20159;
        // N s_55_1: assert s_55_0
        let s_55_1: () = assert!(s_55_0);
        // C s_55_2: const #2s : i
        let s_55_2: i128 = 2;
        // D s_55_3: read-var size:i64
        let s_55_3: i64 = fn_state.size;
        // D s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // D s_55_5: div s_55_4 s_55_2
        let s_55_5: i128 = ((s_55_4) / (s_55_2));
        // D s_55_6: cast reint s_55_5 -> i64
        let s_55_6: i64 = (s_55_5 as i64);
        // D s_55_7: write-var halfsize <= s_55_6
        fn_state.halfsize = s_55_6;
        // C s_55_8: const #() : ()
        let s_55_8: () = ();
        // D s_55_9: create-sum enum = 0:"s_55_8"
        let s_55_9: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_55_8);
        // D s_55_10: read-var halfsize:i64
        let s_55_10: i64 = fn_state.halfsize;
        // D s_55_11: cast zx s_55_10 -> i
        let s_55_11: i128 = (i128::try_from(s_55_10).unwrap());
        // D s_55_12: read-var memaddrdesc:struct
        let s_55_12: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_55_13: read-var accdesc:struct
        let s_55_13: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_55_14: call PhysMemRead(s_55_12, s_55_11, s_55_13, s_55_9)
        let s_55_14: ProductType2b2aba4822138824 = PhysMemRead(
            state,
            tracer,
            s_55_12,
            s_55_11,
            s_55_13,
            s_55_9,
        );
        // D s_55_15: write-var ga#15587 <= s_55_14
        fn_state.ga_15587 = s_55_14;
        // D s_55_16: read-var ga#15587.0:struct
        let s_55_16: ProductTypef8c3639b88223255 = fn_state.ga_15587._0;
        // D s_55_17: read-var ga#15587.1:struct
        let s_55_17: Bits = fn_state.ga_15587._1;
        // D s_55_18: write-var memstatus <= s_55_16
        fn_state.memstatus = s_55_16;
        // D s_55_19: write-var lowhalf <= s_55_17
        fn_state.lowhalf = s_55_17;
        // D s_55_20: read-var memstatus:struct
        let s_55_20: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_55_21: call IsFault__2(s_55_20)
        let s_55_21: bool = IsFault__2(state, tracer, s_55_20);
        // N s_55_22: branch s_55_21 b61 b56
        if s_55_21 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_56_0: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_57_0: read-var memaddrdesc.3:struct
        let s_57_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_57_1: write-var ga#15589 <= s_57_0
        fn_state.ga_15589 = s_57_0;
        // D s_57_2: read-var ga#15589.0:struct
        let s_57_2: u64 = fn_state.ga_15589._0;
        // D s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 56u16);
        // D s_57_4: read-var halfsize:i64
        let s_57_4: i64 = fn_state.halfsize;
        // D s_57_5: cast zx s_57_4 -> i
        let s_57_5: i128 = (i128::try_from(s_57_4).unwrap());
        // D s_57_6: cast cvt s_57_5 -> bv
        let s_57_6: Bits = Bits::new(s_57_5 as u128, 128);
        // D s_57_7: add s_57_3 s_57_6
        let s_57_7: Bits = (s_57_3 + s_57_6);
        // D s_57_8: cast reint s_57_7 -> u56
        let s_57_8: u64 = (s_57_7.value() as u64);
        // D s_57_9: write-var memaddrdesc.3.0 <= s_57_8
        fn_state.memaddrdesc._3._0 = s_57_8;
        // C s_57_10: const #() : ()
        let s_57_10: () = ();
        // D s_57_11: create-sum enum = 0:"s_57_10"
        let s_57_11: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_57_10);
        // D s_57_12: read-var halfsize:i64
        let s_57_12: i64 = fn_state.halfsize;
        // D s_57_13: cast zx s_57_12 -> i
        let s_57_13: i128 = (i128::try_from(s_57_12).unwrap());
        // D s_57_14: read-var memaddrdesc:struct
        let s_57_14: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_57_15: read-var accdesc:struct
        let s_57_15: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_57_16: call PhysMemRead(s_57_14, s_57_13, s_57_15, s_57_11)
        let s_57_16: ProductType2b2aba4822138824 = PhysMemRead(
            state,
            tracer,
            s_57_14,
            s_57_13,
            s_57_15,
            s_57_11,
        );
        // D s_57_17: write-var ga#15592 <= s_57_16
        fn_state.ga_15592 = s_57_16;
        // D s_57_18: read-var ga#15592.0:struct
        let s_57_18: ProductTypef8c3639b88223255 = fn_state.ga_15592._0;
        // D s_57_19: read-var ga#15592.1:struct
        let s_57_19: Bits = fn_state.ga_15592._1;
        // D s_57_20: write-var memstatus <= s_57_18
        fn_state.memstatus = s_57_18;
        // D s_57_21: write-var highhalf <= s_57_19
        fn_state.highhalf = s_57_19;
        // D s_57_22: read-var memstatus:struct
        let s_57_22: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_57_23: call IsFault__2(s_57_22)
        let s_57_23: bool = IsFault__2(state, tracer, s_57_22);
        // N s_57_24: branch s_57_23 b60 b58
        if s_57_23 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_58_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_59_0: read-var highhalf:bv
        let s_59_0: Bits = fn_state.highhalf;
        // D s_59_1: read-var lowhalf:bv
        let s_59_1: Bits = fn_state.lowhalf;
        // D s_59_2: cast reint s_59_0 -> u128
        let s_59_2: u128 = (s_59_0.value() as u128);
        // D s_59_3: size-of s_59_0
        let s_59_3: u16 = s_59_0.length();
        // D s_59_4: cast reint s_59_1 -> u128
        let s_59_4: u128 = (s_59_1.value() as u128);
        // D s_59_5: size-of s_59_1
        let s_59_5: u16 = s_59_1.length();
        // D s_59_6: lsl s_59_2 s_59_5
        let s_59_6: u128 = s_59_2 << s_59_5;
        // D s_59_7: or s_59_6 s_59_4
        let s_59_7: u128 = ((s_59_6) | (s_59_4));
        // D s_59_8: add s_59_3 s_59_5
        let s_59_8: u16 = (s_59_3 + s_59_5);
        // D s_59_9: create-bits s_59_7 s_59_8
        let s_59_9: Bits = Bits::new(s_59_7, s_59_8);
        // D s_59_10: write-var value_name <= s_59_9
        fn_state.value_name = s_59_9;
        // N s_59_11: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_60_0: read-var halfsize:i64
        let s_60_0: i64 = fn_state.halfsize;
        // D s_60_1: cast zx s_60_0 -> i
        let s_60_1: i128 = (i128::try_from(s_60_0).unwrap());
        // D s_60_2: read-var memstatus:struct
        let s_60_2: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_60_3: read-var memaddrdesc:struct
        let s_60_3: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_60_4: read-var accdesc:struct
        let s_60_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_60_5: call HandleExternalReadAbort(s_60_2, s_60_3, s_60_1, s_60_4)
        let s_60_5: () = HandleExternalReadAbort(
            state,
            tracer,
            s_60_2,
            s_60_3,
            s_60_1,
            s_60_4,
        );
        // N s_60_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
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
        // D s_61_5: call HandleExternalReadAbort(s_61_2, s_61_3, s_61_1, s_61_4)
        let s_61_5: () = HandleExternalReadAbort(
            state,
            tracer,
            s_61_2,
            s_61_3,
            s_61_1,
            s_61_4,
        );
        // N s_61_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#20159 <= s_62_0
        fn_state.gs_20159 = s_62_0;
        // N s_62_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_63_0: read-var ispair:u8
        let s_63_0: bool = fn_state.ispair;
        // D s_63_1: write-var gs#20138 <= s_63_0
        fn_state.gs_20138 = s_63_0;
        // N s_63_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
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
        // D s_64_6: call PhysMemRead(s_64_4, s_64_3, s_64_5, s_64_1)
        let s_64_6: ProductType2b2aba4822138824 = PhysMemRead(
            state,
            tracer,
            s_64_4,
            s_64_3,
            s_64_5,
            s_64_1,
        );
        // D s_64_7: write-var ga#15580 <= s_64_6
        fn_state.ga_15580 = s_64_6;
        // D s_64_8: read-var ga#15580.0:struct
        let s_64_8: ProductTypef8c3639b88223255 = fn_state.ga_15580._0;
        // D s_64_9: read-var ga#15580.1:struct
        let s_64_9: Bits = fn_state.ga_15580._1;
        // D s_64_10: write-var memstatus <= s_64_8
        fn_state.memstatus = s_64_8;
        // D s_64_11: write-var value_name <= s_64_9
        fn_state.value_name = s_64_9;
        // D s_64_12: read-var memstatus:struct
        let s_64_12: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_64_13: call IsFault__2(s_64_12)
        let s_64_13: bool = IsFault__2(state, tracer, s_64_12);
        // N s_64_14: branch s_64_13 b67 b65
        if s_64_13 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_65_0: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_66_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_67_0: read-var size:i64
        let s_67_0: i64 = fn_state.size;
        // D s_67_1: cast zx s_67_0 -> i
        let s_67_1: i128 = (i128::try_from(s_67_0).unwrap());
        // D s_67_2: read-var memstatus:struct
        let s_67_2: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_67_3: read-var memaddrdesc:struct
        let s_67_3: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_67_4: read-var accdesc:struct
        let s_67_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_67_5: call HandleExternalReadAbort(s_67_2, s_67_3, s_67_1, s_67_4)
        let s_67_5: () = HandleExternalReadAbort(
            state,
            tracer,
            s_67_2,
            s_67_3,
            s_67_1,
            s_67_4,
        );
        // N s_67_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_68_0: read-var ispair:u8
        let s_68_0: bool = fn_state.ispair;
        // D s_68_1: not s_68_0
        let s_68_1: bool = !s_68_0;
        // D s_68_2: write-var atomic <= s_68_1
        fn_state.atomic = s_68_1;
        // N s_68_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_69_0: read-var aligned:u8
        let s_69_0: bool = fn_state.aligned;
        // D s_69_1: not s_69_0
        let s_69_1: bool = !s_69_0;
        // N s_69_2: branch s_69_1 b74 b70
        if s_69_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#20136 <= s_70_0
        fn_state.gs_20136 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_71_0: read-var gs#20136:u8
        let s_71_0: bool = fn_state.gs_20136;
        // N s_71_1: branch s_71_0 b73 b72
        if s_71_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var atomic <= s_72_0
        fn_state.atomic = s_72_0;
        // N s_72_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_73_0: read-var accdesc:struct
        let s_73_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_73_1: call AlignmentFault(s_73_0)
        let s_73_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_73_0);
        // D s_73_2: read-var address:u64
        let s_73_2: u64 = fn_state.address;
        // D s_73_3: call AArch64_Abort(s_73_2, s_73_1)
        let s_73_3: () = AArch64_Abort(state, tracer, s_73_2, s_73_1);
        // N s_73_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_74_0: const #67u : u32
        let s_74_0: u32 = 67;
        // S s_74_1: call ConstrainUnpredictableBool(s_74_0)
        let s_74_1: bool = ConstrainUnpredictableBool(state, tracer, s_74_0);
        // S s_74_2: not s_74_1
        let s_74_2: bool = !s_74_1;
        // D s_74_3: write-var gs#20136 <= s_74_2
        fn_state.gs_20136 = s_74_2;
        // N s_74_4: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#20135 <= s_75_0
        fn_state.gs_20135 = s_75_0;
        // N s_75_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#20134 <= s_76_0
        fn_state.gs_20134 = s_76_0;
        // N s_76_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // D s_77_1: write-var gs#20133 <= s_77_0
        fn_state.gs_20133 = s_77_0;
        // N s_77_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_78_0: const #1u : u8
        let s_78_0: bool = true;
        // D s_78_1: write-var gs#20132 <= s_78_0
        fn_state.gs_20132 = s_78_0;
        // N s_78_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var atomic <= s_79_0
        fn_state.atomic = s_79_0;
        // N s_79_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_80_0: read-var memaddrdesc.2:struct
        let s_80_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_80_1: write-var ga#15565 <= s_80_0
        fn_state.ga_15565 = s_80_0;
        // D s_80_2: read-var ga#15565.4:struct
        let s_80_2: ProductType183e6678e5239c85 = fn_state.ga_15565._4;
        // D s_80_3: write-var ga#15566 <= s_80_2
        fn_state.ga_15566 = s_80_2;
        // D s_80_4: read-var ga#15566.0:struct
        let s_80_4: u8 = fn_state.ga_15566._0;
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
        // D s_80_10: write-var gs#20130 <= s_80_9
        fn_state.gs_20130 = s_80_9;
        // N s_80_11: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_81_0: read-var memaddrdesc.2:struct
        let s_81_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_81_1: write-var ga#15561 <= s_81_0
        fn_state.ga_15561 = s_81_0;
        // D s_81_2: read-var ga#15561.1:struct
        let s_81_2: ProductType183e6678e5239c85 = fn_state.ga_15561._1;
        // D s_81_3: write-var ga#15562 <= s_81_2
        fn_state.ga_15562 = s_81_2;
        // D s_81_4: read-var ga#15562.0:struct
        let s_81_4: u8 = fn_state.ga_15562._0;
        // D s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 2u16);
        // C s_81_6: const #480u : u32
        let s_81_6: u32 = 480;
        // D s_81_7: read-reg s_81_6:u8
        let s_81_7: u8 = {
            let value = state.read_register::<u8>(s_81_6 as isize);
            tracer.read_register(s_81_6 as isize, value);
            value
        };
        // D s_81_8: cast zx s_81_7 -> bv
        let s_81_8: Bits = Bits::new(s_81_7 as u128, 2u16);
        // D s_81_9: cmp-eq s_81_5 s_81_8
        let s_81_9: bool = ((s_81_5) == (s_81_8));
        // D s_81_10: write-var gs#20129 <= s_81_9
        fn_state.gs_20129 = s_81_9;
        // N s_81_11: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_82_0: const #1u : u8
        let s_82_0: bool = true;
        // D s_82_1: read-var accdesc:struct
        let s_82_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_82_2: read-var memaddrdesc:struct
        let s_82_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_82_3: call SPESampleLoadStore(s_82_0, s_82_1, s_82_2)
        let s_82_3: () = SPESampleLoadStore(state, tracer, s_82_0, s_82_1, s_82_2);
        // N s_82_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_83_0: read-var address:u64
        let s_83_0: u64 = fn_state.address;
        // D s_83_1: call AArch64_PhysicalTag(s_83_0)
        let s_83_1: u8 = AArch64_PhysicalTag(state, tracer, s_83_0);
        // D s_83_2: read-var memaddrdesc:struct
        let s_83_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_83_3: read-var accdesc:struct
        let s_83_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_83_4: call AArch64_CheckTag(s_83_2, s_83_3, s_83_1)
        let s_83_4: bool = AArch64_CheckTag(state, tracer, s_83_2, s_83_3, s_83_1);
        // D s_83_5: write-var ga#15557 <= s_83_4
        fn_state.ga_15557 = s_83_4;
        // N s_83_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_84_0: read-var ga#15557:u8
        let s_84_0: bool = fn_state.ga_15557;
        // D s_84_1: not s_84_0
        let s_84_1: bool = !s_84_0;
        // N s_84_2: branch s_84_1 b87 b85
        if s_84_1 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_85_0: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_86_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_87_0: read-var address:u64
        let s_87_0: u64 = fn_state.address;
        // D s_87_1: read-var accdesc:struct
        let s_87_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_87_2: call AArch64_TagCheckFault(s_87_0, s_87_1)
        let s_87_2: () = AArch64_TagCheckFault(state, tracer, s_87_0, s_87_1);
        // N s_87_3: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_88_0: read-var accdesc.28:struct
        let s_88_0: bool = fn_state.accdesc._28;
        // D s_88_1: write-var gs#20121 <= s_88_0
        fn_state.gs_20121 = s_88_0;
        // N s_88_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_89_0: read-var accdesc.30:struct
        let s_89_0: bool = fn_state.accdesc._30;
        // N s_89_1: branch s_89_0 b95 b90
        if s_89_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#20126 <= s_90_0
        fn_state.gs_20126 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_91_0: read-var gs#20126:u8
        let s_91_0: bool = fn_state.gs_20126;
        // N s_91_1: branch s_91_0 b94 b92
        if s_91_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_92_0: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_93_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_94_0: const #7u : u32
        let s_94_0: u32 = 7;
        // C s_94_1: const #0u : u8
        let s_94_1: bool = false;
        // S s_94_2: call FailTransaction(s_94_0, s_94_1)
        let s_94_2: () = FailTransaction(state, tracer, s_94_0, s_94_1);
        // N s_94_3: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_95_0: read-var memaddrdesc.2:struct
        let s_95_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_95_1: call MemHasTransactionalAccess(s_95_0)
        let s_95_1: bool = MemHasTransactionalAccess(state, tracer, s_95_0);
        // D s_95_2: not s_95_1
        let s_95_2: bool = !s_95_1;
        // D s_95_3: write-var gs#20126 <= s_95_2
        fn_state.gs_20126 = s_95_2;
        // N s_95_4: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
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
    ) -> Bits {
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
    ) -> Bits {
        // D s_98_0: read-var accdesc.28:struct
        let s_98_0: bool = fn_state.accdesc._28;
        // D s_98_1: write-var gs#20117 <= s_98_0
        fn_state.gs_20117 = s_98_0;
        // N s_98_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
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
    ) -> Bits {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // D s_100_1: write-var gs#20112 <= s_100_0
        fn_state.gs_20112 = s_100_0;
        // N s_100_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_101_0: const #1u : u8
        let s_101_0: bool = true;
        // D s_101_1: write-var gs#20113 <= s_101_0
        fn_state.gs_20113 = s_101_0;
        // N s_101_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_102_0: const #1u : u8
        let s_102_0: bool = true;
        // D s_102_1: write-var gs#20114 <= s_102_0
        fn_state.gs_20114 = s_102_0;
        // N s_102_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_103_0: const #1u : u8
        let s_103_0: bool = true;
        // D s_103_1: write-var gs#20115 <= s_103_0
        fn_state.gs_20115 = s_103_0;
        // N s_103_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
