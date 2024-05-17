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
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use Mk_PMSEVFR_EL1_Type::*;
use common::*;
pub fn u__get_PMSEVFR_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        gs_37647: bool,
        gs_37653: bool,
        gs_37648: bool,
        gs_37649: bool,
        gs_37652: bool,
        tmp: ProductType5c790c8ef59cc8b2,
        gs_37644: bool,
        gs_37654: bool,
        gs_37655: bool,
        gs_37656: bool,
        gs_37651: bool,
        gs_37645: bool,
        gs_37646: bool,
        gs_37650: bool,
        gs_37643: bool,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var tmp <= s_0_0
        fn_state.tmp = s_0_0;
        // D s_0_2: read-var tmp.0:struct
        let s_0_2: u64 = fn_state.tmp._0;
        // C s_0_3: const #64s : i
        let s_0_3: i128 = 64;
        // C s_0_4: const #281470681743361u : u48
        let s_0_4: u64 = 281470681743361;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 48u16);
        // D s_0_6: bits-cast zx s_0_5 -> bv length s_0_3
        let s_0_6: Bits = s_0_5.zero_extend(s_0_3);
        // D s_0_7: cast reint s_0_6 -> u64
        let s_0_7: u64 = (s_0_6.value() as u64);
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 64u16);
        // D s_0_9: not s_0_8
        let s_0_9: Bits = !s_0_8;
        // D s_0_10: cast reint s_0_9 -> u64
        let s_0_10: u64 = (s_0_9.value() as u64);
        // D s_0_11: cast zx s_0_2 -> bv
        let s_0_11: Bits = Bits::new(s_0_2 as u128, 64u16);
        // D s_0_12: cast zx s_0_10 -> bv
        let s_0_12: Bits = Bits::new(s_0_10 as u128, 64u16);
        // D s_0_13: and s_0_11 s_0_12
        let s_0_13: Bits = ((s_0_11) & (s_0_12));
        // D s_0_14: cast reint s_0_13 -> u64
        let s_0_14: u64 = (s_0_13.value() as u64);
        // D s_0_15: call Mk_PMSEVFR_EL1_Type(s_0_14)
        let s_0_15: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_0_14,
        );
        // D s_0_16: write-var tmp <= s_0_15
        fn_state.tmp = s_0_15;
        // C s_0_17: const #216u : u32
        let s_0_17: u32 = 216;
        // S s_0_18: call IsFeatureImplemented(s_0_17)
        let s_0_18: bool = IsFeatureImplemented(state, tracer, s_0_17);
        // N s_0_19: branch s_0_18 b84 b1
        if s_0_18 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_1_0: const #"filtering on event 2 is optionally supported" : str
        let s_1_0: &'static str = "filtering on event 2 is optionally supported";
        // S s_1_1: call __IMPDEF_boolean(s_1_0)
        let s_1_1: bool = u__IMPDEF_boolean(state, tracer, s_1_0);
        // D s_1_2: write-var gs#37643 <= s_1_1
        fn_state.gs_37643 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var gs#37643:u8
        let s_2_0: bool = fn_state.gs_37643;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b83 b3
        if s_2_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_4_0: const #216u : u32
        let s_4_0: u32 = 216;
        // S s_4_1: call IsFeatureImplemented(s_4_0)
        let s_4_1: bool = IsFeatureImplemented(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b82 b5
        if s_4_1 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_5_0: const #"filtering on event 4 is optionally supported" : str
        let s_5_0: &'static str = "filtering on event 4 is optionally supported";
        // S s_5_1: call __IMPDEF_boolean(s_5_0)
        let s_5_1: bool = u__IMPDEF_boolean(state, tracer, s_5_0);
        // D s_5_2: write-var gs#37644 <= s_5_1
        fn_state.gs_37644 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_6_0: read-var gs#37644:u8
        let s_6_0: bool = fn_state.gs_37644;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // N s_6_2: branch s_6_1 b81 b7
        if s_6_1 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_8_0: const #164u : u32
        let s_8_0: u32 = 164;
        // S s_8_1: call IsFeatureImplemented(s_8_0)
        let s_8_1: bool = IsFeatureImplemented(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // N s_8_3: branch s_8_2 b80 b9
        if s_8_2 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_10_0: const #216u : u32
        let s_10_0: u32 = 216;
        // S s_10_1: call IsFeatureImplemented(s_10_0)
        let s_10_1: bool = IsFeatureImplemented(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b79 b11
        if s_10_1 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_11_0: const #"filtering on event 8 is optionally supported" : str
        let s_11_0: &'static str = "filtering on event 8 is optionally supported";
        // S s_11_1: call __IMPDEF_boolean(s_11_0)
        let s_11_1: bool = u__IMPDEF_boolean(state, tracer, s_11_0);
        // D s_11_2: write-var gs#37645 <= s_11_1
        fn_state.gs_37645 = s_11_1;
        // N s_11_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_12_0: read-var gs#37645:u8
        let s_12_0: bool = fn_state.gs_37645;
        // N s_12_1: branch s_12_0 b78 b13
        if s_12_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#37646 <= s_13_0
        fn_state.gs_37646 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_14_0: read-var gs#37646:u8
        let s_14_0: bool = fn_state.gs_37646;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b77 b15
        if s_14_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_16_0: const #216u : u32
        let s_16_0: u32 = 216;
        // S s_16_1: call IsFeatureImplemented(s_16_0)
        let s_16_1: bool = IsFeatureImplemented(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b76 b17
        if s_16_1 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_17_0: const #"filtering on event 9 is optionally supported" : str
        let s_17_0: &'static str = "filtering on event 9 is optionally supported";
        // S s_17_1: call __IMPDEF_boolean(s_17_0)
        let s_17_1: bool = u__IMPDEF_boolean(state, tracer, s_17_0);
        // D s_17_2: write-var gs#37647 <= s_17_1
        fn_state.gs_37647 = s_17_1;
        // N s_17_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_18_0: read-var gs#37647:u8
        let s_18_0: bool = fn_state.gs_37647;
        // N s_18_1: branch s_18_0 b75 b19
        if s_18_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#37648 <= s_19_0
        fn_state.gs_37648 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_20_0: read-var gs#37648:u8
        let s_20_0: bool = fn_state.gs_37648;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // N s_20_2: branch s_20_1 b74 b21
        if s_20_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_22_0: const #216u : u32
        let s_22_0: u32 = 216;
        // S s_22_1: call IsFeatureImplemented(s_22_0)
        let s_22_1: bool = IsFeatureImplemented(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b73 b23
        if s_22_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_23_0: const #"filtering on event 10 is optionally supported" : str
        let s_23_0: &'static str = "filtering on event 10 is optionally supported";
        // S s_23_1: call __IMPDEF_boolean(s_23_0)
        let s_23_1: bool = u__IMPDEF_boolean(state, tracer, s_23_0);
        // D s_23_2: write-var gs#37649 <= s_23_1
        fn_state.gs_37649 = s_23_1;
        // N s_23_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_24_0: read-var gs#37649:u8
        let s_24_0: bool = fn_state.gs_37649;
        // N s_24_1: branch s_24_0 b72 b25
        if s_24_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#37650 <= s_25_0
        fn_state.gs_37650 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_26_0: read-var gs#37650:u8
        let s_26_0: bool = fn_state.gs_37650;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // N s_26_2: branch s_26_1 b71 b27
        if s_26_1 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_28_0: const #99u : u32
        let s_28_0: u32 = 99;
        // S s_28_1: call IsFeatureImplemented(s_28_0)
        let s_28_1: bool = IsFeatureImplemented(state, tracer, s_28_0);
        // S s_28_2: not s_28_1
        let s_28_2: bool = !s_28_1;
        // N s_28_3: branch s_28_2 b70 b29
        if s_28_2 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_30_0: const #227u : u32
        let s_30_0: u32 = 227;
        // S s_30_1: call IsFeatureImplemented(s_30_0)
        let s_30_1: bool = IsFeatureImplemented(state, tracer, s_30_0);
        // S s_30_2: not s_30_1
        let s_30_2: bool = !s_30_1;
        // N s_30_3: branch s_30_2 b69 b31
        if s_30_2 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_32_0: const #99u : u32
        let s_32_0: u32 = 99;
        // S s_32_1: call IsFeatureImplemented(s_32_0)
        let s_32_1: bool = IsFeatureImplemented(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b68 b33
        if s_32_1 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#37651 <= s_33_0
        fn_state.gs_37651 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_34_0: read-var gs#37651:u8
        let s_34_0: bool = fn_state.gs_37651;
        // D s_34_1: not s_34_0
        let s_34_1: bool = !s_34_0;
        // N s_34_2: branch s_34_1 b67 b35
        if s_34_1 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_36_0: const #216u : u32
        let s_36_0: u32 = 216;
        // S s_36_1: call IsFeatureImplemented(s_36_0)
        let s_36_1: bool = IsFeatureImplemented(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b66 b37
        if s_36_1 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#37652 <= s_37_0
        fn_state.gs_37652 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_38_0: read-var gs#37652:u8
        let s_38_0: bool = fn_state.gs_37652;
        // D s_38_1: not s_38_0
        let s_38_1: bool = !s_38_0;
        // N s_38_2: branch s_38_1 b65 b39
        if s_38_1 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_40_0: const #216u : u32
        let s_40_0: u32 = 216;
        // S s_40_1: call IsFeatureImplemented(s_40_0)
        let s_40_1: bool = IsFeatureImplemented(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b64 b41
        if s_40_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#37653 <= s_41_0
        fn_state.gs_37653 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_42_0: read-var gs#37653:u8
        let s_42_0: bool = fn_state.gs_37653;
        // D s_42_1: not s_42_0
        let s_42_1: bool = !s_42_0;
        // N s_42_2: branch s_42_1 b63 b43
        if s_42_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_44_0: const #216u : u32
        let s_44_0: u32 = 216;
        // S s_44_1: call IsFeatureImplemented(s_44_0)
        let s_44_1: bool = IsFeatureImplemented(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b62 b45
        if s_44_1 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#37654 <= s_45_0
        fn_state.gs_37654 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_46_0: read-var gs#37654:u8
        let s_46_0: bool = fn_state.gs_37654;
        // D s_46_1: not s_46_0
        let s_46_1: bool = !s_46_0;
        // N s_46_2: branch s_46_1 b61 b47
        if s_46_1 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_48_0: const #216u : u32
        let s_48_0: u32 = 216;
        // S s_48_1: call IsFeatureImplemented(s_48_0)
        let s_48_1: bool = IsFeatureImplemented(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b60 b49
        if s_48_1 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#37655 <= s_49_0
        fn_state.gs_37655 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_50_0: read-var gs#37655:u8
        let s_50_0: bool = fn_state.gs_37655;
        // D s_50_1: not s_50_0
        let s_50_1: bool = !s_50_0;
        // N s_50_2: branch s_50_1 b59 b51
        if s_50_1 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_52_0: const #216u : u32
        let s_52_0: u32 = 216;
        // S s_52_1: call IsFeatureImplemented(s_52_0)
        let s_52_1: bool = IsFeatureImplemented(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b58 b53
        if s_52_1 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#37656 <= s_53_0
        fn_state.gs_37656 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_54_0: read-var gs#37656:u8
        let s_54_0: bool = fn_state.gs_37656;
        // D s_54_1: not s_54_0
        let s_54_1: bool = !s_54_0;
        // N s_54_2: branch s_54_1 b57 b55
        if s_54_1 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_56_0: read-var tmp:struct
        let s_56_0: ProductType5c790c8ef59cc8b2 = fn_state.tmp;
        // N s_56_1: return s_56_0
        return s_56_0;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_57_0: read-var tmp.0:struct
        let s_57_0: u64 = fn_state.tmp._0;
        // C s_57_1: const #64s : i
        let s_57_1: i128 = 64;
        // C s_57_2: const #8388608u : u24
        let s_57_2: u32 = 8388608;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 24u16);
        // D s_57_4: bits-cast zx s_57_3 -> bv length s_57_1
        let s_57_4: Bits = s_57_3.zero_extend(s_57_1);
        // D s_57_5: cast reint s_57_4 -> u64
        let s_57_5: u64 = (s_57_4.value() as u64);
        // D s_57_6: cast zx s_57_5 -> bv
        let s_57_6: Bits = Bits::new(s_57_5 as u128, 64u16);
        // D s_57_7: not s_57_6
        let s_57_7: Bits = !s_57_6;
        // D s_57_8: cast reint s_57_7 -> u64
        let s_57_8: u64 = (s_57_7.value() as u64);
        // D s_57_9: cast zx s_57_0 -> bv
        let s_57_9: Bits = Bits::new(s_57_0 as u128, 64u16);
        // D s_57_10: cast zx s_57_8 -> bv
        let s_57_10: Bits = Bits::new(s_57_8 as u128, 64u16);
        // D s_57_11: and s_57_9 s_57_10
        let s_57_11: Bits = ((s_57_9) & (s_57_10));
        // D s_57_12: cast reint s_57_11 -> u64
        let s_57_12: u64 = (s_57_11.value() as u64);
        // D s_57_13: call Mk_PMSEVFR_EL1_Type(s_57_12)
        let s_57_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_57_12,
        );
        // D s_57_14: write-var tmp <= s_57_13
        fn_state.tmp = s_57_13;
        // N s_57_15: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_58_0: const #"event 23 is implemented" : str
        let s_58_0: &'static str = "event 23 is implemented";
        // S s_58_1: call __IMPDEF_boolean(s_58_0)
        let s_58_1: bool = u__IMPDEF_boolean(state, tracer, s_58_0);
        // D s_58_2: write-var gs#37656 <= s_58_1
        fn_state.gs_37656 = s_58_1;
        // N s_58_3: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_59_0: read-var tmp.0:struct
        let s_59_0: u64 = fn_state.tmp._0;
        // C s_59_1: const #64s : i
        let s_59_1: i128 = 64;
        // C s_59_2: const #4194304u : u24
        let s_59_2: u32 = 4194304;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 24u16);
        // D s_59_4: bits-cast zx s_59_3 -> bv length s_59_1
        let s_59_4: Bits = s_59_3.zero_extend(s_59_1);
        // D s_59_5: cast reint s_59_4 -> u64
        let s_59_5: u64 = (s_59_4.value() as u64);
        // D s_59_6: cast zx s_59_5 -> bv
        let s_59_6: Bits = Bits::new(s_59_5 as u128, 64u16);
        // D s_59_7: not s_59_6
        let s_59_7: Bits = !s_59_6;
        // D s_59_8: cast reint s_59_7 -> u64
        let s_59_8: u64 = (s_59_7.value() as u64);
        // D s_59_9: cast zx s_59_0 -> bv
        let s_59_9: Bits = Bits::new(s_59_0 as u128, 64u16);
        // D s_59_10: cast zx s_59_8 -> bv
        let s_59_10: Bits = Bits::new(s_59_8 as u128, 64u16);
        // D s_59_11: and s_59_9 s_59_10
        let s_59_11: Bits = ((s_59_9) & (s_59_10));
        // D s_59_12: cast reint s_59_11 -> u64
        let s_59_12: u64 = (s_59_11.value() as u64);
        // D s_59_13: call Mk_PMSEVFR_EL1_Type(s_59_12)
        let s_59_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_59_12,
        );
        // D s_59_14: write-var tmp <= s_59_13
        fn_state.tmp = s_59_13;
        // N s_59_15: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_60_0: const #"event 22 is implemented" : str
        let s_60_0: &'static str = "event 22 is implemented";
        // S s_60_1: call __IMPDEF_boolean(s_60_0)
        let s_60_1: bool = u__IMPDEF_boolean(state, tracer, s_60_0);
        // D s_60_2: write-var gs#37655 <= s_60_1
        fn_state.gs_37655 = s_60_1;
        // N s_60_3: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_61_0: read-var tmp.0:struct
        let s_61_0: u64 = fn_state.tmp._0;
        // C s_61_1: const #64s : i
        let s_61_1: i128 = 64;
        // C s_61_2: const #2097152u : u24
        let s_61_2: u32 = 2097152;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 24u16);
        // D s_61_4: bits-cast zx s_61_3 -> bv length s_61_1
        let s_61_4: Bits = s_61_3.zero_extend(s_61_1);
        // D s_61_5: cast reint s_61_4 -> u64
        let s_61_5: u64 = (s_61_4.value() as u64);
        // D s_61_6: cast zx s_61_5 -> bv
        let s_61_6: Bits = Bits::new(s_61_5 as u128, 64u16);
        // D s_61_7: not s_61_6
        let s_61_7: Bits = !s_61_6;
        // D s_61_8: cast reint s_61_7 -> u64
        let s_61_8: u64 = (s_61_7.value() as u64);
        // D s_61_9: cast zx s_61_0 -> bv
        let s_61_9: Bits = Bits::new(s_61_0 as u128, 64u16);
        // D s_61_10: cast zx s_61_8 -> bv
        let s_61_10: Bits = Bits::new(s_61_8 as u128, 64u16);
        // D s_61_11: and s_61_9 s_61_10
        let s_61_11: Bits = ((s_61_9) & (s_61_10));
        // D s_61_12: cast reint s_61_11 -> u64
        let s_61_12: u64 = (s_61_11.value() as u64);
        // D s_61_13: call Mk_PMSEVFR_EL1_Type(s_61_12)
        let s_61_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_61_12,
        );
        // D s_61_14: write-var tmp <= s_61_13
        fn_state.tmp = s_61_13;
        // N s_61_15: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_62_0: const #"event 21 is implemented" : str
        let s_62_0: &'static str = "event 21 is implemented";
        // S s_62_1: call __IMPDEF_boolean(s_62_0)
        let s_62_1: bool = u__IMPDEF_boolean(state, tracer, s_62_0);
        // D s_62_2: write-var gs#37654 <= s_62_1
        fn_state.gs_37654 = s_62_1;
        // N s_62_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_63_0: read-var tmp.0:struct
        let s_63_0: u64 = fn_state.tmp._0;
        // C s_63_1: const #64s : i
        let s_63_1: i128 = 64;
        // C s_63_2: const #1048576u : u24
        let s_63_2: u32 = 1048576;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 24u16);
        // D s_63_4: bits-cast zx s_63_3 -> bv length s_63_1
        let s_63_4: Bits = s_63_3.zero_extend(s_63_1);
        // D s_63_5: cast reint s_63_4 -> u64
        let s_63_5: u64 = (s_63_4.value() as u64);
        // D s_63_6: cast zx s_63_5 -> bv
        let s_63_6: Bits = Bits::new(s_63_5 as u128, 64u16);
        // D s_63_7: not s_63_6
        let s_63_7: Bits = !s_63_6;
        // D s_63_8: cast reint s_63_7 -> u64
        let s_63_8: u64 = (s_63_7.value() as u64);
        // D s_63_9: cast zx s_63_0 -> bv
        let s_63_9: Bits = Bits::new(s_63_0 as u128, 64u16);
        // D s_63_10: cast zx s_63_8 -> bv
        let s_63_10: Bits = Bits::new(s_63_8 as u128, 64u16);
        // D s_63_11: and s_63_9 s_63_10
        let s_63_11: Bits = ((s_63_9) & (s_63_10));
        // D s_63_12: cast reint s_63_11 -> u64
        let s_63_12: u64 = (s_63_11.value() as u64);
        // D s_63_13: call Mk_PMSEVFR_EL1_Type(s_63_12)
        let s_63_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_63_12,
        );
        // D s_63_14: write-var tmp <= s_63_13
        fn_state.tmp = s_63_13;
        // N s_63_15: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_64_0: const #"event 20 is implemented" : str
        let s_64_0: &'static str = "event 20 is implemented";
        // S s_64_1: call __IMPDEF_boolean(s_64_0)
        let s_64_1: bool = u__IMPDEF_boolean(state, tracer, s_64_0);
        // D s_64_2: write-var gs#37653 <= s_64_1
        fn_state.gs_37653 = s_64_1;
        // N s_64_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_65_0: read-var tmp.0:struct
        let s_65_0: u64 = fn_state.tmp._0;
        // C s_65_1: const #64s : i
        let s_65_1: i128 = 64;
        // C s_65_2: const #524288u : u20
        let s_65_2: u32 = 524288;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 20u16);
        // D s_65_4: bits-cast zx s_65_3 -> bv length s_65_1
        let s_65_4: Bits = s_65_3.zero_extend(s_65_1);
        // D s_65_5: cast reint s_65_4 -> u64
        let s_65_5: u64 = (s_65_4.value() as u64);
        // D s_65_6: cast zx s_65_5 -> bv
        let s_65_6: Bits = Bits::new(s_65_5 as u128, 64u16);
        // D s_65_7: not s_65_6
        let s_65_7: Bits = !s_65_6;
        // D s_65_8: cast reint s_65_7 -> u64
        let s_65_8: u64 = (s_65_7.value() as u64);
        // D s_65_9: cast zx s_65_0 -> bv
        let s_65_9: Bits = Bits::new(s_65_0 as u128, 64u16);
        // D s_65_10: cast zx s_65_8 -> bv
        let s_65_10: Bits = Bits::new(s_65_8 as u128, 64u16);
        // D s_65_11: and s_65_9 s_65_10
        let s_65_11: Bits = ((s_65_9) & (s_65_10));
        // D s_65_12: cast reint s_65_11 -> u64
        let s_65_12: u64 = (s_65_11.value() as u64);
        // D s_65_13: call Mk_PMSEVFR_EL1_Type(s_65_12)
        let s_65_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_65_12,
        );
        // D s_65_14: write-var tmp <= s_65_13
        fn_state.tmp = s_65_13;
        // N s_65_15: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_66_0: const #"event 19 is implemented" : str
        let s_66_0: &'static str = "event 19 is implemented";
        // S s_66_1: call __IMPDEF_boolean(s_66_0)
        let s_66_1: bool = u__IMPDEF_boolean(state, tracer, s_66_0);
        // D s_66_2: write-var gs#37652 <= s_66_1
        fn_state.gs_37652 = s_66_1;
        // N s_66_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_67_0: read-var tmp.0:struct
        let s_67_0: u64 = fn_state.tmp._0;
        // C s_67_1: const #64s : i
        let s_67_1: i128 = 64;
        // C s_67_2: const #393216u : u20
        let s_67_2: u32 = 393216;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 20u16);
        // D s_67_4: bits-cast zx s_67_3 -> bv length s_67_1
        let s_67_4: Bits = s_67_3.zero_extend(s_67_1);
        // D s_67_5: cast reint s_67_4 -> u64
        let s_67_5: u64 = (s_67_4.value() as u64);
        // D s_67_6: cast zx s_67_5 -> bv
        let s_67_6: Bits = Bits::new(s_67_5 as u128, 64u16);
        // D s_67_7: not s_67_6
        let s_67_7: Bits = !s_67_6;
        // D s_67_8: cast reint s_67_7 -> u64
        let s_67_8: u64 = (s_67_7.value() as u64);
        // D s_67_9: cast zx s_67_0 -> bv
        let s_67_9: Bits = Bits::new(s_67_0 as u128, 64u16);
        // D s_67_10: cast zx s_67_8 -> bv
        let s_67_10: Bits = Bits::new(s_67_8 as u128, 64u16);
        // D s_67_11: and s_67_9 s_67_10
        let s_67_11: Bits = ((s_67_9) & (s_67_10));
        // D s_67_12: cast reint s_67_11 -> u64
        let s_67_12: u64 = (s_67_11.value() as u64);
        // D s_67_13: call Mk_PMSEVFR_EL1_Type(s_67_12)
        let s_67_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_67_12,
        );
        // D s_67_14: write-var tmp <= s_67_13
        fn_state.tmp = s_67_13;
        // N s_67_15: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_68_0: const #81u : u32
        let s_68_0: u32 = 81;
        // S s_68_1: call IsFeatureImplemented(s_68_0)
        let s_68_1: bool = IsFeatureImplemented(state, tracer, s_68_0);
        // D s_68_2: write-var gs#37651 <= s_68_1
        fn_state.gs_37651 = s_68_1;
        // N s_68_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_69_0: read-var tmp.0:struct
        let s_69_0: u64 = fn_state.tmp._0;
        // C s_69_1: const #64s : i
        let s_69_1: i128 = 64;
        // C s_69_2: const #65536u : u20
        let s_69_2: u32 = 65536;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 20u16);
        // D s_69_4: bits-cast zx s_69_3 -> bv length s_69_1
        let s_69_4: Bits = s_69_3.zero_extend(s_69_1);
        // D s_69_5: cast reint s_69_4 -> u64
        let s_69_5: u64 = (s_69_4.value() as u64);
        // D s_69_6: cast zx s_69_5 -> bv
        let s_69_6: Bits = Bits::new(s_69_5 as u128, 64u16);
        // D s_69_7: not s_69_6
        let s_69_7: Bits = !s_69_6;
        // D s_69_8: cast reint s_69_7 -> u64
        let s_69_8: u64 = (s_69_7.value() as u64);
        // D s_69_9: cast zx s_69_0 -> bv
        let s_69_9: Bits = Bits::new(s_69_0 as u128, 64u16);
        // D s_69_10: cast zx s_69_8 -> bv
        let s_69_10: Bits = Bits::new(s_69_8 as u128, 64u16);
        // D s_69_11: and s_69_9 s_69_10
        let s_69_11: Bits = ((s_69_9) & (s_69_10));
        // D s_69_12: cast reint s_69_11 -> u64
        let s_69_12: u64 = (s_69_11.value() as u64);
        // D s_69_13: call Mk_PMSEVFR_EL1_Type(s_69_12)
        let s_69_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_69_12,
        );
        // D s_69_14: write-var tmp <= s_69_13
        fn_state.tmp = s_69_13;
        // N s_69_15: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_70_0: read-var tmp.0:struct
        let s_70_0: u64 = fn_state.tmp._0;
        // C s_70_1: const #64s : i
        let s_70_1: i128 = 64;
        // C s_70_2: const #2048u : u12
        let s_70_2: u16 = 2048;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 12u16);
        // D s_70_4: bits-cast zx s_70_3 -> bv length s_70_1
        let s_70_4: Bits = s_70_3.zero_extend(s_70_1);
        // D s_70_5: cast reint s_70_4 -> u64
        let s_70_5: u64 = (s_70_4.value() as u64);
        // D s_70_6: cast zx s_70_5 -> bv
        let s_70_6: Bits = Bits::new(s_70_5 as u128, 64u16);
        // D s_70_7: not s_70_6
        let s_70_7: Bits = !s_70_6;
        // D s_70_8: cast reint s_70_7 -> u64
        let s_70_8: u64 = (s_70_7.value() as u64);
        // D s_70_9: cast zx s_70_0 -> bv
        let s_70_9: Bits = Bits::new(s_70_0 as u128, 64u16);
        // D s_70_10: cast zx s_70_8 -> bv
        let s_70_10: Bits = Bits::new(s_70_8 as u128, 64u16);
        // D s_70_11: and s_70_9 s_70_10
        let s_70_11: Bits = ((s_70_9) & (s_70_10));
        // D s_70_12: cast reint s_70_11 -> u64
        let s_70_12: u64 = (s_70_11.value() as u64);
        // D s_70_13: call Mk_PMSEVFR_EL1_Type(s_70_12)
        let s_70_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_70_12,
        );
        // D s_70_14: write-var tmp <= s_70_13
        fn_state.tmp = s_70_13;
        // N s_70_15: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_71_0: read-var tmp.0:struct
        let s_71_0: u64 = fn_state.tmp._0;
        // C s_71_1: const #64s : i
        let s_71_1: i128 = 64;
        // C s_71_2: const #1024u : u12
        let s_71_2: u16 = 1024;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 12u16);
        // D s_71_4: bits-cast zx s_71_3 -> bv length s_71_1
        let s_71_4: Bits = s_71_3.zero_extend(s_71_1);
        // D s_71_5: cast reint s_71_4 -> u64
        let s_71_5: u64 = (s_71_4.value() as u64);
        // D s_71_6: cast zx s_71_5 -> bv
        let s_71_6: Bits = Bits::new(s_71_5 as u128, 64u16);
        // D s_71_7: not s_71_6
        let s_71_7: Bits = !s_71_6;
        // D s_71_8: cast reint s_71_7 -> u64
        let s_71_8: u64 = (s_71_7.value() as u64);
        // D s_71_9: cast zx s_71_0 -> bv
        let s_71_9: Bits = Bits::new(s_71_0 as u128, 64u16);
        // D s_71_10: cast zx s_71_8 -> bv
        let s_71_10: Bits = Bits::new(s_71_8 as u128, 64u16);
        // D s_71_11: and s_71_9 s_71_10
        let s_71_11: Bits = ((s_71_9) & (s_71_10));
        // D s_71_12: cast reint s_71_11 -> u64
        let s_71_12: u64 = (s_71_11.value() as u64);
        // D s_71_13: call Mk_PMSEVFR_EL1_Type(s_71_12)
        let s_71_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_71_12,
        );
        // D s_71_14: write-var tmp <= s_71_13
        fn_state.tmp = s_71_13;
        // N s_71_15: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_72_0: const #"event 10 is implemented" : str
        let s_72_0: &'static str = "event 10 is implemented";
        // S s_72_1: call __IMPDEF_boolean(s_72_0)
        let s_72_1: bool = u__IMPDEF_boolean(state, tracer, s_72_0);
        // D s_72_2: write-var gs#37650 <= s_72_1
        fn_state.gs_37650 = s_72_1;
        // N s_72_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#37649 <= s_73_0
        fn_state.gs_37649 = s_73_0;
        // N s_73_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_74_0: read-var tmp.0:struct
        let s_74_0: u64 = fn_state.tmp._0;
        // C s_74_1: const #64s : i
        let s_74_1: i128 = 64;
        // C s_74_2: const #512u : u12
        let s_74_2: u16 = 512;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 12u16);
        // D s_74_4: bits-cast zx s_74_3 -> bv length s_74_1
        let s_74_4: Bits = s_74_3.zero_extend(s_74_1);
        // D s_74_5: cast reint s_74_4 -> u64
        let s_74_5: u64 = (s_74_4.value() as u64);
        // D s_74_6: cast zx s_74_5 -> bv
        let s_74_6: Bits = Bits::new(s_74_5 as u128, 64u16);
        // D s_74_7: not s_74_6
        let s_74_7: Bits = !s_74_6;
        // D s_74_8: cast reint s_74_7 -> u64
        let s_74_8: u64 = (s_74_7.value() as u64);
        // D s_74_9: cast zx s_74_0 -> bv
        let s_74_9: Bits = Bits::new(s_74_0 as u128, 64u16);
        // D s_74_10: cast zx s_74_8 -> bv
        let s_74_10: Bits = Bits::new(s_74_8 as u128, 64u16);
        // D s_74_11: and s_74_9 s_74_10
        let s_74_11: Bits = ((s_74_9) & (s_74_10));
        // D s_74_12: cast reint s_74_11 -> u64
        let s_74_12: u64 = (s_74_11.value() as u64);
        // D s_74_13: call Mk_PMSEVFR_EL1_Type(s_74_12)
        let s_74_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_74_12,
        );
        // D s_74_14: write-var tmp <= s_74_13
        fn_state.tmp = s_74_13;
        // N s_74_15: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_75_0: const #"event 9 is implemented" : str
        let s_75_0: &'static str = "event 9 is implemented";
        // S s_75_1: call __IMPDEF_boolean(s_75_0)
        let s_75_1: bool = u__IMPDEF_boolean(state, tracer, s_75_0);
        // D s_75_2: write-var gs#37648 <= s_75_1
        fn_state.gs_37648 = s_75_1;
        // N s_75_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#37647 <= s_76_0
        fn_state.gs_37647 = s_76_0;
        // N s_76_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_77_0: read-var tmp.0:struct
        let s_77_0: u64 = fn_state.tmp._0;
        // C s_77_1: const #64s : i
        let s_77_1: i128 = 64;
        // C s_77_2: const #256u : u12
        let s_77_2: u16 = 256;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 12u16);
        // D s_77_4: bits-cast zx s_77_3 -> bv length s_77_1
        let s_77_4: Bits = s_77_3.zero_extend(s_77_1);
        // D s_77_5: cast reint s_77_4 -> u64
        let s_77_5: u64 = (s_77_4.value() as u64);
        // D s_77_6: cast zx s_77_5 -> bv
        let s_77_6: Bits = Bits::new(s_77_5 as u128, 64u16);
        // D s_77_7: not s_77_6
        let s_77_7: Bits = !s_77_6;
        // D s_77_8: cast reint s_77_7 -> u64
        let s_77_8: u64 = (s_77_7.value() as u64);
        // D s_77_9: cast zx s_77_0 -> bv
        let s_77_9: Bits = Bits::new(s_77_0 as u128, 64u16);
        // D s_77_10: cast zx s_77_8 -> bv
        let s_77_10: Bits = Bits::new(s_77_8 as u128, 64u16);
        // D s_77_11: and s_77_9 s_77_10
        let s_77_11: Bits = ((s_77_9) & (s_77_10));
        // D s_77_12: cast reint s_77_11 -> u64
        let s_77_12: u64 = (s_77_11.value() as u64);
        // D s_77_13: call Mk_PMSEVFR_EL1_Type(s_77_12)
        let s_77_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_77_12,
        );
        // D s_77_14: write-var tmp <= s_77_13
        fn_state.tmp = s_77_13;
        // N s_77_15: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_78_0: const #"event 8 is implemented" : str
        let s_78_0: &'static str = "event 8 is implemented";
        // S s_78_1: call __IMPDEF_boolean(s_78_0)
        let s_78_1: bool = u__IMPDEF_boolean(state, tracer, s_78_0);
        // D s_78_2: write-var gs#37646 <= s_78_1
        fn_state.gs_37646 = s_78_1;
        // N s_78_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var gs#37645 <= s_79_0
        fn_state.gs_37645 = s_79_0;
        // N s_79_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_80_0: read-var tmp.0:struct
        let s_80_0: u64 = fn_state.tmp._0;
        // C s_80_1: const #64s : i
        let s_80_1: i128 = 64;
        // C s_80_2: const #64u : u8
        let s_80_2: u8 = 64;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 8u16);
        // D s_80_4: bits-cast zx s_80_3 -> bv length s_80_1
        let s_80_4: Bits = s_80_3.zero_extend(s_80_1);
        // D s_80_5: cast reint s_80_4 -> u64
        let s_80_5: u64 = (s_80_4.value() as u64);
        // D s_80_6: cast zx s_80_5 -> bv
        let s_80_6: Bits = Bits::new(s_80_5 as u128, 64u16);
        // D s_80_7: not s_80_6
        let s_80_7: Bits = !s_80_6;
        // D s_80_8: cast reint s_80_7 -> u64
        let s_80_8: u64 = (s_80_7.value() as u64);
        // D s_80_9: cast zx s_80_0 -> bv
        let s_80_9: Bits = Bits::new(s_80_0 as u128, 64u16);
        // D s_80_10: cast zx s_80_8 -> bv
        let s_80_10: Bits = Bits::new(s_80_8 as u128, 64u16);
        // D s_80_11: and s_80_9 s_80_10
        let s_80_11: Bits = ((s_80_9) & (s_80_10));
        // D s_80_12: cast reint s_80_11 -> u64
        let s_80_12: u64 = (s_80_11.value() as u64);
        // D s_80_13: call Mk_PMSEVFR_EL1_Type(s_80_12)
        let s_80_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_80_12,
        );
        // D s_80_14: write-var tmp <= s_80_13
        fn_state.tmp = s_80_13;
        // N s_80_15: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_81_0: read-var tmp.0:struct
        let s_81_0: u64 = fn_state.tmp._0;
        // C s_81_1: const #64s : i
        let s_81_1: i128 = 64;
        // C s_81_2: const #16u : u8
        let s_81_2: u8 = 16;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 8u16);
        // D s_81_4: bits-cast zx s_81_3 -> bv length s_81_1
        let s_81_4: Bits = s_81_3.zero_extend(s_81_1);
        // D s_81_5: cast reint s_81_4 -> u64
        let s_81_5: u64 = (s_81_4.value() as u64);
        // D s_81_6: cast zx s_81_5 -> bv
        let s_81_6: Bits = Bits::new(s_81_5 as u128, 64u16);
        // D s_81_7: not s_81_6
        let s_81_7: Bits = !s_81_6;
        // D s_81_8: cast reint s_81_7 -> u64
        let s_81_8: u64 = (s_81_7.value() as u64);
        // D s_81_9: cast zx s_81_0 -> bv
        let s_81_9: Bits = Bits::new(s_81_0 as u128, 64u16);
        // D s_81_10: cast zx s_81_8 -> bv
        let s_81_10: Bits = Bits::new(s_81_8 as u128, 64u16);
        // D s_81_11: and s_81_9 s_81_10
        let s_81_11: Bits = ((s_81_9) & (s_81_10));
        // D s_81_12: cast reint s_81_11 -> u64
        let s_81_12: u64 = (s_81_11.value() as u64);
        // D s_81_13: call Mk_PMSEVFR_EL1_Type(s_81_12)
        let s_81_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_81_12,
        );
        // D s_81_14: write-var tmp <= s_81_13
        fn_state.tmp = s_81_13;
        // N s_81_15: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_82_0: const #1u : u8
        let s_82_0: bool = true;
        // D s_82_1: write-var gs#37644 <= s_82_0
        fn_state.gs_37644 = s_82_0;
        // N s_82_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_83_0: read-var tmp.0:struct
        let s_83_0: u64 = fn_state.tmp._0;
        // C s_83_1: const #64s : i
        let s_83_1: i128 = 64;
        // C s_83_2: const #4u : u8
        let s_83_2: u8 = 4;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 4u16);
        // D s_83_4: bits-cast zx s_83_3 -> bv length s_83_1
        let s_83_4: Bits = s_83_3.zero_extend(s_83_1);
        // D s_83_5: cast reint s_83_4 -> u64
        let s_83_5: u64 = (s_83_4.value() as u64);
        // D s_83_6: cast zx s_83_5 -> bv
        let s_83_6: Bits = Bits::new(s_83_5 as u128, 64u16);
        // D s_83_7: not s_83_6
        let s_83_7: Bits = !s_83_6;
        // D s_83_8: cast reint s_83_7 -> u64
        let s_83_8: u64 = (s_83_7.value() as u64);
        // D s_83_9: cast zx s_83_0 -> bv
        let s_83_9: Bits = Bits::new(s_83_0 as u128, 64u16);
        // D s_83_10: cast zx s_83_8 -> bv
        let s_83_10: Bits = Bits::new(s_83_8 as u128, 64u16);
        // D s_83_11: and s_83_9 s_83_10
        let s_83_11: Bits = ((s_83_9) & (s_83_10));
        // D s_83_12: cast reint s_83_11 -> u64
        let s_83_12: u64 = (s_83_11.value() as u64);
        // D s_83_13: call Mk_PMSEVFR_EL1_Type(s_83_12)
        let s_83_13: ProductType5c790c8ef59cc8b2 = Mk_PMSEVFR_EL1_Type(
            state,
            tracer,
            s_83_12,
        );
        // D s_83_14: write-var tmp <= s_83_13
        fn_state.tmp = s_83_13;
        // N s_83_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_84_0: const #1u : u8
        let s_84_0: bool = true;
        // D s_84_1: write-var gs#37643 <= s_84_0
        fn_state.gs_37643 = s_84_0;
        // N s_84_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
