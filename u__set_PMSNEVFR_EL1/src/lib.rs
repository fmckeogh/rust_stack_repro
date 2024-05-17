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
use u_get_PMSNEVFR_EL1_Type_E24::*;
use u_get_PMSNEVFR_EL1_Type_E60::*;
use u_get_PMSNEVFR_EL1_Type_E49::*;
use u_get_PMSNEVFR_EL1_Type_E58::*;
use u_get_PMSNEVFR_EL1_Type_E57::*;
use u_get_PMSNEVFR_EL1_Type_E61::*;
use u_get_PMSNEVFR_EL1_Type_E52::*;
use u_get_PMSNEVFR_EL1_Type_E30::*;
use u_get_PMSNEVFR_EL1_Type_E62::*;
use u_get_PMSNEVFR_EL1_Type_E55::*;
use u_get_PMSNEVFR_EL1_Type_E12::*;
use u_get_PMSNEVFR_EL1_Type_E59::*;
use u_get_PMSNEVFR_EL1_Type_E56::*;
use u_get_PMSNEVFR_EL1_Type_E51::*;
use u_get_PMSNEVFR_EL1_Type_E13::*;
use u_get_PMSNEVFR_EL1_Type_E63::*;
use u_get_PMSNEVFR_EL1_Type_E15::*;
use u_get_PMSNEVFR_EL1_Type_E25::*;
use u_get_PMSNEVFR_EL1_Type_E29::*;
use u__IMPDEF_boolean::*;
use u_get_PMSNEVFR_EL1_Type_E48::*;
use u_get_PMSNEVFR_EL1_Type_E27::*;
use u_get_PMSNEVFR_EL1_Type_E31::*;
use u_get_PMSNEVFR_EL1_Type_E54::*;
use u_get_PMSNEVFR_EL1_Type_E14::*;
use u_get_PMSNEVFR_EL1_Type_E28::*;
use u_get_PMSNEVFR_EL1_Type_E50::*;
use u_get_PMSNEVFR_EL1_Type_E26::*;
use u_get_PMSNEVFR_EL1_Type_E53::*;
use common::*;
pub fn u__set_PMSNEVFR_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
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
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #"event 63 is not implemented, or filtering on event 63 is not supported" : str
        let s_0_2: &'static str = "event 63 is not implemented, or filtering on event 63 is not supported";
        // S s_0_3: call __IMPDEF_boolean(s_0_2)
        let s_0_3: bool = u__IMPDEF_boolean(state, tracer, s_0_2);
        // S s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b84 b1
        if s_0_4 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #"event 62 is not implemented, or filtering on event 62 is not supported" : str
        let s_2_0: &'static str = "event 62 is not implemented, or filtering on event 62 is not supported";
        // S s_2_1: call __IMPDEF_boolean(s_2_0)
        let s_2_1: bool = u__IMPDEF_boolean(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // N s_2_3: branch s_2_2 b83 b3
        if s_2_2 {
            return block_83(state, tracer, fn_state);
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
        // C s_4_0: const #"event 61 is not implemented, or filtering on event 61 is not supported" : str
        let s_4_0: &'static str = "event 61 is not implemented, or filtering on event 61 is not supported";
        // S s_4_1: call __IMPDEF_boolean(s_4_0)
        let s_4_1: bool = u__IMPDEF_boolean(state, tracer, s_4_0);
        // S s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // N s_4_3: branch s_4_2 b82 b5
        if s_4_2 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #"event 60 is not implemented, or filtering on event 60 is not supported" : str
        let s_6_0: &'static str = "event 60 is not implemented, or filtering on event 60 is not supported";
        // S s_6_1: call __IMPDEF_boolean(s_6_0)
        let s_6_1: bool = u__IMPDEF_boolean(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // N s_6_3: branch s_6_2 b81 b7
        if s_6_2 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #"event 59 is not implemented, or filtering on event 59 is not supported" : str
        let s_8_0: &'static str = "event 59 is not implemented, or filtering on event 59 is not supported";
        // S s_8_1: call __IMPDEF_boolean(s_8_0)
        let s_8_1: bool = u__IMPDEF_boolean(state, tracer, s_8_0);
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
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #"event 58 is not implemented, or filtering on event 58 is not supported" : str
        let s_10_0: &'static str = "event 58 is not implemented, or filtering on event 58 is not supported";
        // S s_10_1: call __IMPDEF_boolean(s_10_0)
        let s_10_1: bool = u__IMPDEF_boolean(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // N s_10_3: branch s_10_2 b79 b11
        if s_10_2 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #"event 57 is not implemented, or filtering on event 57 is not supported" : str
        let s_12_0: &'static str = "event 57 is not implemented, or filtering on event 57 is not supported";
        // S s_12_1: call __IMPDEF_boolean(s_12_0)
        let s_12_1: bool = u__IMPDEF_boolean(state, tracer, s_12_0);
        // S s_12_2: not s_12_1
        let s_12_2: bool = !s_12_1;
        // N s_12_3: branch s_12_2 b78 b13
        if s_12_2 {
            return block_78(state, tracer, fn_state);
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
        // C s_14_0: const #"event 56 is not implemented, or filtering on event 56 is not supported" : str
        let s_14_0: &'static str = "event 56 is not implemented, or filtering on event 56 is not supported";
        // S s_14_1: call __IMPDEF_boolean(s_14_0)
        let s_14_1: bool = u__IMPDEF_boolean(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // N s_14_3: branch s_14_2 b77 b15
        if s_14_2 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #"event 55 is not implemented, or filtering on event 55 is not supported" : str
        let s_16_0: &'static str = "event 55 is not implemented, or filtering on event 55 is not supported";
        // S s_16_1: call __IMPDEF_boolean(s_16_0)
        let s_16_1: bool = u__IMPDEF_boolean(state, tracer, s_16_0);
        // S s_16_2: not s_16_1
        let s_16_2: bool = !s_16_1;
        // N s_16_3: branch s_16_2 b76 b17
        if s_16_2 {
            return block_76(state, tracer, fn_state);
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
        // C s_18_0: const #"event 54 is not implemented, or filtering on event 54 is not supported" : str
        let s_18_0: &'static str = "event 54 is not implemented, or filtering on event 54 is not supported";
        // S s_18_1: call __IMPDEF_boolean(s_18_0)
        let s_18_1: bool = u__IMPDEF_boolean(state, tracer, s_18_0);
        // S s_18_2: not s_18_1
        let s_18_2: bool = !s_18_1;
        // N s_18_3: branch s_18_2 b75 b19
        if s_18_2 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #"event 53 is not implemented, or filtering on event 53 is not supported" : str
        let s_20_0: &'static str = "event 53 is not implemented, or filtering on event 53 is not supported";
        // S s_20_1: call __IMPDEF_boolean(s_20_0)
        let s_20_1: bool = u__IMPDEF_boolean(state, tracer, s_20_0);
        // S s_20_2: not s_20_1
        let s_20_2: bool = !s_20_1;
        // N s_20_3: branch s_20_2 b74 b21
        if s_20_2 {
            return block_74(state, tracer, fn_state);
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
        // C s_22_0: const #"event 52 is not implemented, or filtering on event 52 is not supported" : str
        let s_22_0: &'static str = "event 52 is not implemented, or filtering on event 52 is not supported";
        // S s_22_1: call __IMPDEF_boolean(s_22_0)
        let s_22_1: bool = u__IMPDEF_boolean(state, tracer, s_22_0);
        // S s_22_2: not s_22_1
        let s_22_2: bool = !s_22_1;
        // N s_22_3: branch s_22_2 b73 b23
        if s_22_2 {
            return block_73(state, tracer, fn_state);
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
        // C s_24_0: const #"event 51 is not implemented, or filtering on event 51 is not supported" : str
        let s_24_0: &'static str = "event 51 is not implemented, or filtering on event 51 is not supported";
        // S s_24_1: call __IMPDEF_boolean(s_24_0)
        let s_24_1: bool = u__IMPDEF_boolean(state, tracer, s_24_0);
        // S s_24_2: not s_24_1
        let s_24_2: bool = !s_24_1;
        // N s_24_3: branch s_24_2 b72 b25
        if s_24_2 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #"event 50 is not implemented, or filtering on event 50 is not supported" : str
        let s_26_0: &'static str = "event 50 is not implemented, or filtering on event 50 is not supported";
        // S s_26_1: call __IMPDEF_boolean(s_26_0)
        let s_26_1: bool = u__IMPDEF_boolean(state, tracer, s_26_0);
        // S s_26_2: not s_26_1
        let s_26_2: bool = !s_26_1;
        // N s_26_3: branch s_26_2 b71 b27
        if s_26_2 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #"event 49 is not implemented, or filtering on event 49 is not supported" : str
        let s_28_0: &'static str = "event 49 is not implemented, or filtering on event 49 is not supported";
        // S s_28_1: call __IMPDEF_boolean(s_28_0)
        let s_28_1: bool = u__IMPDEF_boolean(state, tracer, s_28_0);
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
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #"event 48 is not implemented, or filtering on event 48 is not supported" : str
        let s_30_0: &'static str = "event 48 is not implemented, or filtering on event 48 is not supported";
        // S s_30_1: call __IMPDEF_boolean(s_30_0)
        let s_30_1: bool = u__IMPDEF_boolean(state, tracer, s_30_0);
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
    ) -> () {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #"event 31 is not implemented, or filtering on event 31 is not supported" : str
        let s_32_0: &'static str = "event 31 is not implemented, or filtering on event 31 is not supported";
        // S s_32_1: call __IMPDEF_boolean(s_32_0)
        let s_32_1: bool = u__IMPDEF_boolean(state, tracer, s_32_0);
        // S s_32_2: not s_32_1
        let s_32_2: bool = !s_32_1;
        // N s_32_3: branch s_32_2 b68 b33
        if s_32_2 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #"event 30 is not implemented, or filtering on event 30 is not supported" : str
        let s_34_0: &'static str = "event 30 is not implemented, or filtering on event 30 is not supported";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // S s_34_2: not s_34_1
        let s_34_2: bool = !s_34_1;
        // N s_34_3: branch s_34_2 b67 b35
        if s_34_2 {
            return block_67(state, tracer, fn_state);
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
        // C s_36_0: const #"event 29 is not implemented, or filtering on event 29 is not supported" : str
        let s_36_0: &'static str = "event 29 is not implemented, or filtering on event 29 is not supported";
        // S s_36_1: call __IMPDEF_boolean(s_36_0)
        let s_36_1: bool = u__IMPDEF_boolean(state, tracer, s_36_0);
        // S s_36_2: not s_36_1
        let s_36_2: bool = !s_36_1;
        // N s_36_3: branch s_36_2 b66 b37
        if s_36_2 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #"event 28 is not implemented, or filtering on event 28 is not supported" : str
        let s_38_0: &'static str = "event 28 is not implemented, or filtering on event 28 is not supported";
        // S s_38_1: call __IMPDEF_boolean(s_38_0)
        let s_38_1: bool = u__IMPDEF_boolean(state, tracer, s_38_0);
        // S s_38_2: not s_38_1
        let s_38_2: bool = !s_38_1;
        // N s_38_3: branch s_38_2 b65 b39
        if s_38_2 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #"event 27 is not implemented, or filtering on event 27 is not supported" : str
        let s_40_0: &'static str = "event 27 is not implemented, or filtering on event 27 is not supported";
        // S s_40_1: call __IMPDEF_boolean(s_40_0)
        let s_40_1: bool = u__IMPDEF_boolean(state, tracer, s_40_0);
        // S s_40_2: not s_40_1
        let s_40_2: bool = !s_40_1;
        // N s_40_3: branch s_40_2 b64 b41
        if s_40_2 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #"event 26 is not implemented, or filtering on event 26 is not supported" : str
        let s_42_0: &'static str = "event 26 is not implemented, or filtering on event 26 is not supported";
        // S s_42_1: call __IMPDEF_boolean(s_42_0)
        let s_42_1: bool = u__IMPDEF_boolean(state, tracer, s_42_0);
        // S s_42_2: not s_42_1
        let s_42_2: bool = !s_42_1;
        // N s_42_3: branch s_42_2 b63 b43
        if s_42_2 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
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
        // C s_44_0: const #"event 25 is not implemented, or filtering on event 25 is not supported" : str
        let s_44_0: &'static str = "event 25 is not implemented, or filtering on event 25 is not supported";
        // S s_44_1: call __IMPDEF_boolean(s_44_0)
        let s_44_1: bool = u__IMPDEF_boolean(state, tracer, s_44_0);
        // S s_44_2: not s_44_1
        let s_44_2: bool = !s_44_1;
        // N s_44_3: branch s_44_2 b62 b45
        if s_44_2 {
            return block_62(state, tracer, fn_state);
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
        // C s_46_0: const #"event 24 is not implemented, or filtering on event 24 is not supported" : str
        let s_46_0: &'static str = "event 24 is not implemented, or filtering on event 24 is not supported";
        // S s_46_1: call __IMPDEF_boolean(s_46_0)
        let s_46_1: bool = u__IMPDEF_boolean(state, tracer, s_46_0);
        // S s_46_2: not s_46_1
        let s_46_2: bool = !s_46_1;
        // N s_46_3: branch s_46_2 b61 b47
        if s_46_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #"event 15 is not implemented, or filtering on event 15 is not supported" : str
        let s_48_0: &'static str = "event 15 is not implemented, or filtering on event 15 is not supported";
        // S s_48_1: call __IMPDEF_boolean(s_48_0)
        let s_48_1: bool = u__IMPDEF_boolean(state, tracer, s_48_0);
        // S s_48_2: not s_48_1
        let s_48_2: bool = !s_48_1;
        // N s_48_3: branch s_48_2 b60 b49
        if s_48_2 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #"event 14 is not implemented, or filtering on event 14 is not supported" : str
        let s_50_0: &'static str = "event 14 is not implemented, or filtering on event 14 is not supported";
        // S s_50_1: call __IMPDEF_boolean(s_50_0)
        let s_50_1: bool = u__IMPDEF_boolean(state, tracer, s_50_0);
        // S s_50_2: not s_50_1
        let s_50_2: bool = !s_50_1;
        // N s_50_3: branch s_50_2 b59 b51
        if s_50_2 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #"event 13 is not implemented, or filtering on event 13 is not supported" : str
        let s_52_0: &'static str = "event 13 is not implemented, or filtering on event 13 is not supported";
        // S s_52_1: call __IMPDEF_boolean(s_52_0)
        let s_52_1: bool = u__IMPDEF_boolean(state, tracer, s_52_0);
        // S s_52_2: not s_52_1
        let s_52_2: bool = !s_52_1;
        // N s_52_3: branch s_52_2 b58 b53
        if s_52_2 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #"event 12 is not implemented, or filtering on event 12 is not supported" : str
        let s_54_0: &'static str = "event 12 is not implemented, or filtering on event 12 is not supported";
        // S s_54_1: call __IMPDEF_boolean(s_54_0)
        let s_54_1: bool = u__IMPDEF_boolean(state, tracer, s_54_0);
        // S s_54_2: not s_54_1
        let s_54_2: bool = !s_54_1;
        // N s_54_3: branch s_54_2 b57 b55
        if s_54_2 {
            return block_57(state, tracer, fn_state);
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
        // D s_56_0: read-var r:struct
        let s_56_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // C s_56_1: const #12904u : u32
        let s_56_1: u32 = 12904;
        // N s_56_2: write-reg s_56_1 <= s_56_0
        let s_56_2: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_56_1 as isize, s_56_0);
            tracer.write_register(s_56_1 as isize, s_56_0);
        };
        // N s_56_3: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #12904u : u32
        let s_57_0: u32 = 12904;
        // D s_57_1: read-reg s_57_0:struct
        let s_57_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call _get_PMSNEVFR_EL1_Type_E12(s_57_1)
        let s_57_2: bool = u_get_PMSNEVFR_EL1_Type_E12(state, tracer, s_57_1);
        // D s_57_3: read-var r:struct
        let s_57_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_57_4: write-var r <= s_57_3
        fn_state.r = s_57_3;
        // N s_57_5: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #12904u : u32
        let s_58_0: u32 = 12904;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_PMSNEVFR_EL1_Type_E13(s_58_1)
        let s_58_2: bool = u_get_PMSNEVFR_EL1_Type_E13(state, tracer, s_58_1);
        // D s_58_3: read-var r:struct
        let s_58_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_58_4: write-var r <= s_58_3
        fn_state.r = s_58_3;
        // N s_58_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #12904u : u32
        let s_59_0: u32 = 12904;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_PMSNEVFR_EL1_Type_E14(s_59_1)
        let s_59_2: bool = u_get_PMSNEVFR_EL1_Type_E14(state, tracer, s_59_1);
        // D s_59_3: read-var r:struct
        let s_59_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_59_4: write-var r <= s_59_3
        fn_state.r = s_59_3;
        // N s_59_5: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #12904u : u32
        let s_60_0: u32 = 12904;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_PMSNEVFR_EL1_Type_E15(s_60_1)
        let s_60_2: bool = u_get_PMSNEVFR_EL1_Type_E15(state, tracer, s_60_1);
        // D s_60_3: read-var r:struct
        let s_60_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_60_4: write-var r <= s_60_3
        fn_state.r = s_60_3;
        // N s_60_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #12904u : u32
        let s_61_0: u32 = 12904;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_PMSNEVFR_EL1_Type_E24(s_61_1)
        let s_61_2: bool = u_get_PMSNEVFR_EL1_Type_E24(state, tracer, s_61_1);
        // D s_61_3: read-var r:struct
        let s_61_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_61_4: write-var r <= s_61_3
        fn_state.r = s_61_3;
        // N s_61_5: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #12904u : u32
        let s_62_0: u32 = 12904;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_PMSNEVFR_EL1_Type_E25(s_62_1)
        let s_62_2: bool = u_get_PMSNEVFR_EL1_Type_E25(state, tracer, s_62_1);
        // D s_62_3: read-var r:struct
        let s_62_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_62_4: write-var r <= s_62_3
        fn_state.r = s_62_3;
        // N s_62_5: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #12904u : u32
        let s_63_0: u32 = 12904;
        // D s_63_1: read-reg s_63_0:struct
        let s_63_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call _get_PMSNEVFR_EL1_Type_E26(s_63_1)
        let s_63_2: bool = u_get_PMSNEVFR_EL1_Type_E26(state, tracer, s_63_1);
        // D s_63_3: read-var r:struct
        let s_63_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_63_4: write-var r <= s_63_3
        fn_state.r = s_63_3;
        // N s_63_5: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #12904u : u32
        let s_64_0: u32 = 12904;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_PMSNEVFR_EL1_Type_E27(s_64_1)
        let s_64_2: bool = u_get_PMSNEVFR_EL1_Type_E27(state, tracer, s_64_1);
        // D s_64_3: read-var r:struct
        let s_64_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_64_4: write-var r <= s_64_3
        fn_state.r = s_64_3;
        // N s_64_5: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #12904u : u32
        let s_65_0: u32 = 12904;
        // D s_65_1: read-reg s_65_0:struct
        let s_65_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call _get_PMSNEVFR_EL1_Type_E28(s_65_1)
        let s_65_2: bool = u_get_PMSNEVFR_EL1_Type_E28(state, tracer, s_65_1);
        // D s_65_3: read-var r:struct
        let s_65_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_65_4: write-var r <= s_65_3
        fn_state.r = s_65_3;
        // N s_65_5: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #12904u : u32
        let s_66_0: u32 = 12904;
        // D s_66_1: read-reg s_66_0:struct
        let s_66_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call _get_PMSNEVFR_EL1_Type_E29(s_66_1)
        let s_66_2: bool = u_get_PMSNEVFR_EL1_Type_E29(state, tracer, s_66_1);
        // D s_66_3: read-var r:struct
        let s_66_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_66_4: write-var r <= s_66_3
        fn_state.r = s_66_3;
        // N s_66_5: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #12904u : u32
        let s_67_0: u32 = 12904;
        // D s_67_1: read-reg s_67_0:struct
        let s_67_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: call _get_PMSNEVFR_EL1_Type_E30(s_67_1)
        let s_67_2: bool = u_get_PMSNEVFR_EL1_Type_E30(state, tracer, s_67_1);
        // D s_67_3: read-var r:struct
        let s_67_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_67_4: write-var r <= s_67_3
        fn_state.r = s_67_3;
        // N s_67_5: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #12904u : u32
        let s_68_0: u32 = 12904;
        // D s_68_1: read-reg s_68_0:struct
        let s_68_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call _get_PMSNEVFR_EL1_Type_E31(s_68_1)
        let s_68_2: bool = u_get_PMSNEVFR_EL1_Type_E31(state, tracer, s_68_1);
        // D s_68_3: read-var r:struct
        let s_68_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_68_4: write-var r <= s_68_3
        fn_state.r = s_68_3;
        // N s_68_5: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #12904u : u32
        let s_69_0: u32 = 12904;
        // D s_69_1: read-reg s_69_0:struct
        let s_69_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call _get_PMSNEVFR_EL1_Type_E48(s_69_1)
        let s_69_2: bool = u_get_PMSNEVFR_EL1_Type_E48(state, tracer, s_69_1);
        // D s_69_3: read-var r:struct
        let s_69_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_69_4: write-var r <= s_69_3
        fn_state.r = s_69_3;
        // N s_69_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #12904u : u32
        let s_70_0: u32 = 12904;
        // D s_70_1: read-reg s_70_0:struct
        let s_70_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call _get_PMSNEVFR_EL1_Type_E49(s_70_1)
        let s_70_2: bool = u_get_PMSNEVFR_EL1_Type_E49(state, tracer, s_70_1);
        // D s_70_3: read-var r:struct
        let s_70_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_70_4: write-var r <= s_70_3
        fn_state.r = s_70_3;
        // N s_70_5: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #12904u : u32
        let s_71_0: u32 = 12904;
        // D s_71_1: read-reg s_71_0:struct
        let s_71_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // D s_71_2: call _get_PMSNEVFR_EL1_Type_E50(s_71_1)
        let s_71_2: bool = u_get_PMSNEVFR_EL1_Type_E50(state, tracer, s_71_1);
        // D s_71_3: read-var r:struct
        let s_71_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_71_4: write-var r <= s_71_3
        fn_state.r = s_71_3;
        // N s_71_5: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #12904u : u32
        let s_72_0: u32 = 12904;
        // D s_72_1: read-reg s_72_0:struct
        let s_72_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: call _get_PMSNEVFR_EL1_Type_E51(s_72_1)
        let s_72_2: bool = u_get_PMSNEVFR_EL1_Type_E51(state, tracer, s_72_1);
        // D s_72_3: read-var r:struct
        let s_72_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_72_4: write-var r <= s_72_3
        fn_state.r = s_72_3;
        // N s_72_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #12904u : u32
        let s_73_0: u32 = 12904;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call _get_PMSNEVFR_EL1_Type_E52(s_73_1)
        let s_73_2: bool = u_get_PMSNEVFR_EL1_Type_E52(state, tracer, s_73_1);
        // D s_73_3: read-var r:struct
        let s_73_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_73_4: write-var r <= s_73_3
        fn_state.r = s_73_3;
        // N s_73_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #12904u : u32
        let s_74_0: u32 = 12904;
        // D s_74_1: read-reg s_74_0:struct
        let s_74_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call _get_PMSNEVFR_EL1_Type_E53(s_74_1)
        let s_74_2: bool = u_get_PMSNEVFR_EL1_Type_E53(state, tracer, s_74_1);
        // D s_74_3: read-var r:struct
        let s_74_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_74_4: write-var r <= s_74_3
        fn_state.r = s_74_3;
        // N s_74_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #12904u : u32
        let s_75_0: u32 = 12904;
        // D s_75_1: read-reg s_75_0:struct
        let s_75_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: call _get_PMSNEVFR_EL1_Type_E54(s_75_1)
        let s_75_2: bool = u_get_PMSNEVFR_EL1_Type_E54(state, tracer, s_75_1);
        // D s_75_3: read-var r:struct
        let s_75_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_75_4: write-var r <= s_75_3
        fn_state.r = s_75_3;
        // N s_75_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #12904u : u32
        let s_76_0: u32 = 12904;
        // D s_76_1: read-reg s_76_0:struct
        let s_76_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call _get_PMSNEVFR_EL1_Type_E55(s_76_1)
        let s_76_2: bool = u_get_PMSNEVFR_EL1_Type_E55(state, tracer, s_76_1);
        // D s_76_3: read-var r:struct
        let s_76_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_76_4: write-var r <= s_76_3
        fn_state.r = s_76_3;
        // N s_76_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #12904u : u32
        let s_77_0: u32 = 12904;
        // D s_77_1: read-reg s_77_0:struct
        let s_77_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call _get_PMSNEVFR_EL1_Type_E56(s_77_1)
        let s_77_2: bool = u_get_PMSNEVFR_EL1_Type_E56(state, tracer, s_77_1);
        // D s_77_3: read-var r:struct
        let s_77_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_77_4: write-var r <= s_77_3
        fn_state.r = s_77_3;
        // N s_77_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #12904u : u32
        let s_78_0: u32 = 12904;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_PMSNEVFR_EL1_Type_E57(s_78_1)
        let s_78_2: bool = u_get_PMSNEVFR_EL1_Type_E57(state, tracer, s_78_1);
        // D s_78_3: read-var r:struct
        let s_78_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_78_4: write-var r <= s_78_3
        fn_state.r = s_78_3;
        // N s_78_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #12904u : u32
        let s_79_0: u32 = 12904;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_PMSNEVFR_EL1_Type_E58(s_79_1)
        let s_79_2: bool = u_get_PMSNEVFR_EL1_Type_E58(state, tracer, s_79_1);
        // D s_79_3: read-var r:struct
        let s_79_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_79_4: write-var r <= s_79_3
        fn_state.r = s_79_3;
        // N s_79_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #12904u : u32
        let s_80_0: u32 = 12904;
        // D s_80_1: read-reg s_80_0:struct
        let s_80_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call _get_PMSNEVFR_EL1_Type_E59(s_80_1)
        let s_80_2: bool = u_get_PMSNEVFR_EL1_Type_E59(state, tracer, s_80_1);
        // D s_80_3: read-var r:struct
        let s_80_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_80_4: write-var r <= s_80_3
        fn_state.r = s_80_3;
        // N s_80_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #12904u : u32
        let s_81_0: u32 = 12904;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_PMSNEVFR_EL1_Type_E60(s_81_1)
        let s_81_2: bool = u_get_PMSNEVFR_EL1_Type_E60(state, tracer, s_81_1);
        // D s_81_3: read-var r:struct
        let s_81_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_81_4: write-var r <= s_81_3
        fn_state.r = s_81_3;
        // N s_81_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #12904u : u32
        let s_82_0: u32 = 12904;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_PMSNEVFR_EL1_Type_E61(s_82_1)
        let s_82_2: bool = u_get_PMSNEVFR_EL1_Type_E61(state, tracer, s_82_1);
        // D s_82_3: read-var r:struct
        let s_82_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_82_4: write-var r <= s_82_3
        fn_state.r = s_82_3;
        // N s_82_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #12904u : u32
        let s_83_0: u32 = 12904;
        // D s_83_1: read-reg s_83_0:struct
        let s_83_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call _get_PMSNEVFR_EL1_Type_E62(s_83_1)
        let s_83_2: bool = u_get_PMSNEVFR_EL1_Type_E62(state, tracer, s_83_1);
        // D s_83_3: read-var r:struct
        let s_83_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_83_4: write-var r <= s_83_3
        fn_state.r = s_83_3;
        // N s_83_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #12904u : u32
        let s_84_0: u32 = 12904;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call _get_PMSNEVFR_EL1_Type_E63(s_84_1)
        let s_84_2: bool = u_get_PMSNEVFR_EL1_Type_E63(state, tracer, s_84_1);
        // D s_84_3: read-var r:struct
        let s_84_3: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_84_4: write-var r <= s_84_3
        fn_state.r = s_84_3;
        // N s_84_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
