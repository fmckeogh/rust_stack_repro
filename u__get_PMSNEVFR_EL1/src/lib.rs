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
use Mk_PMSNEVFR_EL1_Type::*;
use common::*;
pub fn u__get_PMSNEVFR_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        gs_37693: bool,
        tmp: ProductType5c790c8ef59cc8b2,
        gs_37697: bool,
        gs_37699: bool,
        gs_37691: bool,
        gs_37696: bool,
        gs_37701: bool,
        gs_37688: bool,
        gs_37694: bool,
        gs_37695: bool,
        gs_37698: bool,
        gs_37692: bool,
        gs_37700: bool,
        gs_37690: bool,
        gs_37689: bool,
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
        // D s_0_15: call Mk_PMSNEVFR_EL1_Type(s_0_14)
        let s_0_15: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_0_14,
        );
        // D s_0_16: write-var tmp <= s_0_15
        fn_state.tmp = s_0_15;
        // C s_0_17: const #"the PE supports sampling of speculative instructions" : str
        let s_0_17: &'static str = "the PE supports sampling of speculative instructions";
        // S s_0_18: call __IMPDEF_boolean(s_0_17)
        let s_0_18: bool = u__IMPDEF_boolean(state, tracer, s_0_17);
        // S s_0_19: not s_0_18
        let s_0_19: bool = !s_0_18;
        // N s_0_20: branch s_0_19 b168 b1
        if s_0_19 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_2_0: const #216u : u32
        let s_2_0: u32 = 216;
        // S s_2_1: call IsFeatureImplemented(s_2_0)
        let s_2_1: bool = IsFeatureImplemented(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b167 b3
        if s_2_1 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_3_0: const #"filtering on event 2 is optionally supported" : str
        let s_3_0: &'static str = "filtering on event 2 is optionally supported";
        // S s_3_1: call __IMPDEF_boolean(s_3_0)
        let s_3_1: bool = u__IMPDEF_boolean(state, tracer, s_3_0);
        // D s_3_2: write-var gs#37688 <= s_3_1
        fn_state.gs_37688 = s_3_1;
        // N s_3_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_4_0: read-var gs#37688:u8
        let s_4_0: bool = fn_state.gs_37688;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b166 b5
        if s_4_1 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_6_0: const #216u : u32
        let s_6_0: u32 = 216;
        // S s_6_1: call IsFeatureImplemented(s_6_0)
        let s_6_1: bool = IsFeatureImplemented(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b165 b7
        if s_6_1 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_7_0: const #"filtering on event 4 is optionally supported" : str
        let s_7_0: &'static str = "filtering on event 4 is optionally supported";
        // S s_7_1: call __IMPDEF_boolean(s_7_0)
        let s_7_1: bool = u__IMPDEF_boolean(state, tracer, s_7_0);
        // D s_7_2: write-var gs#37689 <= s_7_1
        fn_state.gs_37689 = s_7_1;
        // N s_7_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_8_0: read-var gs#37689:u8
        let s_8_0: bool = fn_state.gs_37689;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b164 b9
        if s_8_1 {
            return block_164(state, tracer, fn_state);
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
        // N s_10_2: branch s_10_1 b163 b11
        if s_10_1 {
            return block_163(state, tracer, fn_state);
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
        // D s_11_2: write-var gs#37690 <= s_11_1
        fn_state.gs_37690 = s_11_1;
        // N s_11_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_12_0: read-var gs#37690:u8
        let s_12_0: bool = fn_state.gs_37690;
        // N s_12_1: branch s_12_0 b162 b13
        if s_12_0 {
            return block_162(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#37691 <= s_13_0
        fn_state.gs_37691 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_14_0: read-var gs#37691:u8
        let s_14_0: bool = fn_state.gs_37691;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b161 b15
        if s_14_1 {
            return block_161(state, tracer, fn_state);
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
        // N s_16_2: branch s_16_1 b160 b17
        if s_16_1 {
            return block_160(state, tracer, fn_state);
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
        // D s_17_2: write-var gs#37692 <= s_17_1
        fn_state.gs_37692 = s_17_1;
        // N s_17_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_18_0: read-var gs#37692:u8
        let s_18_0: bool = fn_state.gs_37692;
        // N s_18_1: branch s_18_0 b159 b19
        if s_18_0 {
            return block_159(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#37693 <= s_19_0
        fn_state.gs_37693 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_20_0: read-var gs#37693:u8
        let s_20_0: bool = fn_state.gs_37693;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // N s_20_2: branch s_20_1 b158 b21
        if s_20_1 {
            return block_158(state, tracer, fn_state);
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
        // N s_22_2: branch s_22_1 b157 b23
        if s_22_1 {
            return block_157(state, tracer, fn_state);
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
        // D s_23_2: write-var gs#37694 <= s_23_1
        fn_state.gs_37694 = s_23_1;
        // N s_23_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_24_0: read-var gs#37694:u8
        let s_24_0: bool = fn_state.gs_37694;
        // N s_24_1: branch s_24_0 b156 b25
        if s_24_0 {
            return block_156(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#37695 <= s_25_0
        fn_state.gs_37695 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_26_0: read-var gs#37695:u8
        let s_26_0: bool = fn_state.gs_37695;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // N s_26_2: branch s_26_1 b155 b27
        if s_26_1 {
            return block_155(state, tracer, fn_state);
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
        // N s_28_3: branch s_28_2 b154 b29
        if s_28_2 {
            return block_154(state, tracer, fn_state);
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
        // C s_30_0: const #"event 12 is not implemented, or filtering on event 12 is not supported" : str
        let s_30_0: &'static str = "event 12 is not implemented, or filtering on event 12 is not supported";
        // S s_30_1: call __IMPDEF_boolean(s_30_0)
        let s_30_1: bool = u__IMPDEF_boolean(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b153 b31
        if s_30_1 {
            return block_153(state, tracer, fn_state);
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
        // C s_32_0: const #"event 13 is not implemented, or filtering on event 13 is not supported" : str
        let s_32_0: &'static str = "event 13 is not implemented, or filtering on event 13 is not supported";
        // S s_32_1: call __IMPDEF_boolean(s_32_0)
        let s_32_1: bool = u__IMPDEF_boolean(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b152 b33
        if s_32_1 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_34_0: const #"event 14 is not implemented, or filtering on event 14 is not supported" : str
        let s_34_0: &'static str = "event 14 is not implemented, or filtering on event 14 is not supported";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b151 b35
        if s_34_1 {
            return block_151(state, tracer, fn_state);
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
        // C s_36_0: const #"event 15 is not implemented, or filtering on event 15 is not supported" : str
        let s_36_0: &'static str = "event 15 is not implemented, or filtering on event 15 is not supported";
        // S s_36_1: call __IMPDEF_boolean(s_36_0)
        let s_36_1: bool = u__IMPDEF_boolean(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b150 b37
        if s_36_1 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_38_0: const #227u : u32
        let s_38_0: u32 = 227;
        // S s_38_1: call IsFeatureImplemented(s_38_0)
        let s_38_1: bool = IsFeatureImplemented(state, tracer, s_38_0);
        // S s_38_2: not s_38_1
        let s_38_2: bool = !s_38_1;
        // N s_38_3: branch s_38_2 b149 b39
        if s_38_2 {
            return block_149(state, tracer, fn_state);
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
        // C s_40_0: const #81u : u32
        let s_40_0: u32 = 81;
        // S s_40_1: call IsFeatureImplemented(s_40_0)
        let s_40_1: bool = IsFeatureImplemented(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b148 b41
        if s_40_1 {
            return block_148(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#37696 <= s_41_0
        fn_state.gs_37696 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_42_0: read-var gs#37696:u8
        let s_42_0: bool = fn_state.gs_37696;
        // D s_42_1: not s_42_0
        let s_42_1: bool = !s_42_0;
        // N s_42_2: branch s_42_1 b147 b43
        if s_42_1 {
            return block_147(state, tracer, fn_state);
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
        // N s_44_2: branch s_44_1 b146 b45
        if s_44_1 {
            return block_146(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#37697 <= s_45_0
        fn_state.gs_37697 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_46_0: read-var gs#37697:u8
        let s_46_0: bool = fn_state.gs_37697;
        // D s_46_1: not s_46_0
        let s_46_1: bool = !s_46_0;
        // N s_46_2: branch s_46_1 b145 b47
        if s_46_1 {
            return block_145(state, tracer, fn_state);
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
        // N s_48_2: branch s_48_1 b144 b49
        if s_48_1 {
            return block_144(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#37698 <= s_49_0
        fn_state.gs_37698 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_50_0: read-var gs#37698:u8
        let s_50_0: bool = fn_state.gs_37698;
        // D s_50_1: not s_50_0
        let s_50_1: bool = !s_50_0;
        // N s_50_2: branch s_50_1 b143 b51
        if s_50_1 {
            return block_143(state, tracer, fn_state);
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
        // N s_52_2: branch s_52_1 b142 b53
        if s_52_1 {
            return block_142(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#37699 <= s_53_0
        fn_state.gs_37699 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_54_0: read-var gs#37699:u8
        let s_54_0: bool = fn_state.gs_37699;
        // D s_54_1: not s_54_0
        let s_54_1: bool = !s_54_0;
        // N s_54_2: branch s_54_1 b141 b55
        if s_54_1 {
            return block_141(state, tracer, fn_state);
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
        // C s_56_0: const #216u : u32
        let s_56_0: u32 = 216;
        // S s_56_1: call IsFeatureImplemented(s_56_0)
        let s_56_1: bool = IsFeatureImplemented(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b140 b57
        if s_56_1 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#37700 <= s_57_0
        fn_state.gs_37700 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_58_0: read-var gs#37700:u8
        let s_58_0: bool = fn_state.gs_37700;
        // D s_58_1: not s_58_0
        let s_58_1: bool = !s_58_0;
        // N s_58_2: branch s_58_1 b139 b59
        if s_58_1 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_59_0: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_60_0: const #216u : u32
        let s_60_0: u32 = 216;
        // S s_60_1: call IsFeatureImplemented(s_60_0)
        let s_60_1: bool = IsFeatureImplemented(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b138 b61
        if s_60_1 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#37701 <= s_61_0
        fn_state.gs_37701 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_62_0: read-var gs#37701:u8
        let s_62_0: bool = fn_state.gs_37701;
        // D s_62_1: not s_62_0
        let s_62_1: bool = !s_62_0;
        // N s_62_2: branch s_62_1 b137 b63
        if s_62_1 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_63_0: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_64_0: const #"event 24 is not implemented, or filtering on event 24 is not supported" : str
        let s_64_0: &'static str = "event 24 is not implemented, or filtering on event 24 is not supported";
        // S s_64_1: call __IMPDEF_boolean(s_64_0)
        let s_64_1: bool = u__IMPDEF_boolean(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b136 b65
        if s_64_1 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_65_0: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_66_0: const #"event 25 is not implemented, or filtering on event 25 is not supported" : str
        let s_66_0: &'static str = "event 25 is not implemented, or filtering on event 25 is not supported";
        // S s_66_1: call __IMPDEF_boolean(s_66_0)
        let s_66_1: bool = u__IMPDEF_boolean(state, tracer, s_66_0);
        // N s_66_2: branch s_66_1 b135 b67
        if s_66_1 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_67_0: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_68_0: const #"event 26 is not implemented, or filtering on event 26 is not supported" : str
        let s_68_0: &'static str = "event 26 is not implemented, or filtering on event 26 is not supported";
        // S s_68_1: call __IMPDEF_boolean(s_68_0)
        let s_68_1: bool = u__IMPDEF_boolean(state, tracer, s_68_0);
        // N s_68_2: branch s_68_1 b134 b69
        if s_68_1 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_69_0: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_70_0: const #"event 27 is not implemented, or filtering on event 27 is not supported" : str
        let s_70_0: &'static str = "event 27 is not implemented, or filtering on event 27 is not supported";
        // S s_70_1: call __IMPDEF_boolean(s_70_0)
        let s_70_1: bool = u__IMPDEF_boolean(state, tracer, s_70_0);
        // N s_70_2: branch s_70_1 b133 b71
        if s_70_1 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_71_0: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_72_0: const #"event 28 is not implemented, or filtering on event 28 is not supported" : str
        let s_72_0: &'static str = "event 28 is not implemented, or filtering on event 28 is not supported";
        // S s_72_1: call __IMPDEF_boolean(s_72_0)
        let s_72_1: bool = u__IMPDEF_boolean(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b132 b73
        if s_72_1 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_73_0: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_74_0: const #"event 29 is not implemented, or filtering on event 29 is not supported" : str
        let s_74_0: &'static str = "event 29 is not implemented, or filtering on event 29 is not supported";
        // S s_74_1: call __IMPDEF_boolean(s_74_0)
        let s_74_1: bool = u__IMPDEF_boolean(state, tracer, s_74_0);
        // N s_74_2: branch s_74_1 b131 b75
        if s_74_1 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_75_0: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_76_0: const #"event 30 is not implemented, or filtering on event 30 is not supported" : str
        let s_76_0: &'static str = "event 30 is not implemented, or filtering on event 30 is not supported";
        // S s_76_1: call __IMPDEF_boolean(s_76_0)
        let s_76_1: bool = u__IMPDEF_boolean(state, tracer, s_76_0);
        // N s_76_2: branch s_76_1 b130 b77
        if s_76_1 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_77_0: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_78_0: const #"event 31 is not implemented, or filtering on event 31 is not supported" : str
        let s_78_0: &'static str = "event 31 is not implemented, or filtering on event 31 is not supported";
        // S s_78_1: call __IMPDEF_boolean(s_78_0)
        let s_78_1: bool = u__IMPDEF_boolean(state, tracer, s_78_0);
        // N s_78_2: branch s_78_1 b129 b79
        if s_78_1 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_79_0: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_80_0: const #"event 48 is not implemented, or filtering on event 48 is not supported" : str
        let s_80_0: &'static str = "event 48 is not implemented, or filtering on event 48 is not supported";
        // S s_80_1: call __IMPDEF_boolean(s_80_0)
        let s_80_1: bool = u__IMPDEF_boolean(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b128 b81
        if s_80_1 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_81_0: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_82_0: const #"event 49 is not implemented, or filtering on event 49 is not supported" : str
        let s_82_0: &'static str = "event 49 is not implemented, or filtering on event 49 is not supported";
        // S s_82_1: call __IMPDEF_boolean(s_82_0)
        let s_82_1: bool = u__IMPDEF_boolean(state, tracer, s_82_0);
        // N s_82_2: branch s_82_1 b127 b83
        if s_82_1 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_83_0: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_84_0: const #"event 50 is not implemented, or filtering on event 50 is not supported" : str
        let s_84_0: &'static str = "event 50 is not implemented, or filtering on event 50 is not supported";
        // S s_84_1: call __IMPDEF_boolean(s_84_0)
        let s_84_1: bool = u__IMPDEF_boolean(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b126 b85
        if s_84_1 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_85_0: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_86_0: const #"event 51 is not implemented, or filtering on event 51 is not supported" : str
        let s_86_0: &'static str = "event 51 is not implemented, or filtering on event 51 is not supported";
        // S s_86_1: call __IMPDEF_boolean(s_86_0)
        let s_86_1: bool = u__IMPDEF_boolean(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b125 b87
        if s_86_1 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_87_0: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_88_0: const #"event 52 is not implemented, or filtering on event 52 is not supported" : str
        let s_88_0: &'static str = "event 52 is not implemented, or filtering on event 52 is not supported";
        // S s_88_1: call __IMPDEF_boolean(s_88_0)
        let s_88_1: bool = u__IMPDEF_boolean(state, tracer, s_88_0);
        // N s_88_2: branch s_88_1 b124 b89
        if s_88_1 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_89_0: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_90_0: const #"event 53 is not implemented, or filtering on event 53 is not supported" : str
        let s_90_0: &'static str = "event 53 is not implemented, or filtering on event 53 is not supported";
        // S s_90_1: call __IMPDEF_boolean(s_90_0)
        let s_90_1: bool = u__IMPDEF_boolean(state, tracer, s_90_0);
        // N s_90_2: branch s_90_1 b123 b91
        if s_90_1 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_91_0: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_92_0: const #"event 54 is not implemented, or filtering on event 54 is not supported" : str
        let s_92_0: &'static str = "event 54 is not implemented, or filtering on event 54 is not supported";
        // S s_92_1: call __IMPDEF_boolean(s_92_0)
        let s_92_1: bool = u__IMPDEF_boolean(state, tracer, s_92_0);
        // N s_92_2: branch s_92_1 b122 b93
        if s_92_1 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_93_0: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_94_0: const #"event 55 is not implemented, or filtering on event 55 is not supported" : str
        let s_94_0: &'static str = "event 55 is not implemented, or filtering on event 55 is not supported";
        // S s_94_1: call __IMPDEF_boolean(s_94_0)
        let s_94_1: bool = u__IMPDEF_boolean(state, tracer, s_94_0);
        // N s_94_2: branch s_94_1 b121 b95
        if s_94_1 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_95_0: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_96_0: const #"event 56 is not implemented, or filtering on event 56 is not supported" : str
        let s_96_0: &'static str = "event 56 is not implemented, or filtering on event 56 is not supported";
        // S s_96_1: call __IMPDEF_boolean(s_96_0)
        let s_96_1: bool = u__IMPDEF_boolean(state, tracer, s_96_0);
        // N s_96_2: branch s_96_1 b120 b97
        if s_96_1 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_97_0: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_98_0: const #"event 57 is not implemented, or filtering on event 57 is not supported" : str
        let s_98_0: &'static str = "event 57 is not implemented, or filtering on event 57 is not supported";
        // S s_98_1: call __IMPDEF_boolean(s_98_0)
        let s_98_1: bool = u__IMPDEF_boolean(state, tracer, s_98_0);
        // N s_98_2: branch s_98_1 b119 b99
        if s_98_1 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_99_0: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_100_0: const #"event 58 is not implemented, or filtering on event 58 is not supported" : str
        let s_100_0: &'static str = "event 58 is not implemented, or filtering on event 58 is not supported";
        // S s_100_1: call __IMPDEF_boolean(s_100_0)
        let s_100_1: bool = u__IMPDEF_boolean(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b118 b101
        if s_100_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_101_0: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_102_0: const #"event 59 is not implemented, or filtering on event 59 is not supported" : str
        let s_102_0: &'static str = "event 59 is not implemented, or filtering on event 59 is not supported";
        // S s_102_1: call __IMPDEF_boolean(s_102_0)
        let s_102_1: bool = u__IMPDEF_boolean(state, tracer, s_102_0);
        // N s_102_2: branch s_102_1 b117 b103
        if s_102_1 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_103_0: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_104_0: const #"event 60 is not implemented, or filtering on event 60 is not supported" : str
        let s_104_0: &'static str = "event 60 is not implemented, or filtering on event 60 is not supported";
        // S s_104_1: call __IMPDEF_boolean(s_104_0)
        let s_104_1: bool = u__IMPDEF_boolean(state, tracer, s_104_0);
        // N s_104_2: branch s_104_1 b116 b105
        if s_104_1 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_105_0: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_106_0: const #"event 61 is not implemented, or filtering on event 61 is not supported" : str
        let s_106_0: &'static str = "event 61 is not implemented, or filtering on event 61 is not supported";
        // S s_106_1: call __IMPDEF_boolean(s_106_0)
        let s_106_1: bool = u__IMPDEF_boolean(state, tracer, s_106_0);
        // N s_106_2: branch s_106_1 b115 b107
        if s_106_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_107_0: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_108_0: const #"event 62 is not implemented, or filtering on event 62 is not supported" : str
        let s_108_0: &'static str = "event 62 is not implemented, or filtering on event 62 is not supported";
        // S s_108_1: call __IMPDEF_boolean(s_108_0)
        let s_108_1: bool = u__IMPDEF_boolean(state, tracer, s_108_0);
        // N s_108_2: branch s_108_1 b114 b109
        if s_108_1 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_109_0: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_110_0: const #"event 63 is not implemented, or filtering on event 63 is not supported" : str
        let s_110_0: &'static str = "event 63 is not implemented, or filtering on event 63 is not supported";
        // S s_110_1: call __IMPDEF_boolean(s_110_0)
        let s_110_1: bool = u__IMPDEF_boolean(state, tracer, s_110_0);
        // N s_110_2: branch s_110_1 b113 b111
        if s_110_1 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_111_0: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_112_0: read-var tmp:struct
        let s_112_0: ProductType5c790c8ef59cc8b2 = fn_state.tmp;
        // N s_112_1: return s_112_0
        return s_112_0;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_113_0: read-var tmp.0:struct
        let s_113_0: u64 = fn_state.tmp._0;
        // C s_113_1: const #9223372036854775808u : u64
        let s_113_1: u64 = 9223372036854775808;
        // C s_113_2: cast zx s_113_1 -> bv
        let s_113_2: Bits = Bits::new(s_113_1 as u128, 64u16);
        // C s_113_3: not s_113_2
        let s_113_3: Bits = !s_113_2;
        // C s_113_4: cast reint s_113_3 -> u64
        let s_113_4: u64 = (s_113_3.value() as u64);
        // D s_113_5: cast zx s_113_0 -> bv
        let s_113_5: Bits = Bits::new(s_113_0 as u128, 64u16);
        // C s_113_6: cast zx s_113_4 -> bv
        let s_113_6: Bits = Bits::new(s_113_4 as u128, 64u16);
        // D s_113_7: and s_113_5 s_113_6
        let s_113_7: Bits = ((s_113_5) & (s_113_6));
        // D s_113_8: cast reint s_113_7 -> u64
        let s_113_8: u64 = (s_113_7.value() as u64);
        // D s_113_9: call Mk_PMSNEVFR_EL1_Type(s_113_8)
        let s_113_9: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_113_8,
        );
        // D s_113_10: write-var tmp <= s_113_9
        fn_state.tmp = s_113_9;
        // N s_113_11: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_114_0: read-var tmp.0:struct
        let s_114_0: u64 = fn_state.tmp._0;
        // C s_114_1: const #4611686018427387904u : u64
        let s_114_1: u64 = 4611686018427387904;
        // C s_114_2: cast zx s_114_1 -> bv
        let s_114_2: Bits = Bits::new(s_114_1 as u128, 64u16);
        // C s_114_3: not s_114_2
        let s_114_3: Bits = !s_114_2;
        // C s_114_4: cast reint s_114_3 -> u64
        let s_114_4: u64 = (s_114_3.value() as u64);
        // D s_114_5: cast zx s_114_0 -> bv
        let s_114_5: Bits = Bits::new(s_114_0 as u128, 64u16);
        // C s_114_6: cast zx s_114_4 -> bv
        let s_114_6: Bits = Bits::new(s_114_4 as u128, 64u16);
        // D s_114_7: and s_114_5 s_114_6
        let s_114_7: Bits = ((s_114_5) & (s_114_6));
        // D s_114_8: cast reint s_114_7 -> u64
        let s_114_8: u64 = (s_114_7.value() as u64);
        // D s_114_9: call Mk_PMSNEVFR_EL1_Type(s_114_8)
        let s_114_9: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_114_8,
        );
        // D s_114_10: write-var tmp <= s_114_9
        fn_state.tmp = s_114_9;
        // N s_114_11: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_115_0: read-var tmp.0:struct
        let s_115_0: u64 = fn_state.tmp._0;
        // C s_115_1: const #2305843009213693952u : u64
        let s_115_1: u64 = 2305843009213693952;
        // C s_115_2: cast zx s_115_1 -> bv
        let s_115_2: Bits = Bits::new(s_115_1 as u128, 64u16);
        // C s_115_3: not s_115_2
        let s_115_3: Bits = !s_115_2;
        // C s_115_4: cast reint s_115_3 -> u64
        let s_115_4: u64 = (s_115_3.value() as u64);
        // D s_115_5: cast zx s_115_0 -> bv
        let s_115_5: Bits = Bits::new(s_115_0 as u128, 64u16);
        // C s_115_6: cast zx s_115_4 -> bv
        let s_115_6: Bits = Bits::new(s_115_4 as u128, 64u16);
        // D s_115_7: and s_115_5 s_115_6
        let s_115_7: Bits = ((s_115_5) & (s_115_6));
        // D s_115_8: cast reint s_115_7 -> u64
        let s_115_8: u64 = (s_115_7.value() as u64);
        // D s_115_9: call Mk_PMSNEVFR_EL1_Type(s_115_8)
        let s_115_9: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_115_8,
        );
        // D s_115_10: write-var tmp <= s_115_9
        fn_state.tmp = s_115_9;
        // N s_115_11: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_116_0: read-var tmp.0:struct
        let s_116_0: u64 = fn_state.tmp._0;
        // C s_116_1: const #1152921504606846976u : u64
        let s_116_1: u64 = 1152921504606846976;
        // C s_116_2: cast zx s_116_1 -> bv
        let s_116_2: Bits = Bits::new(s_116_1 as u128, 64u16);
        // C s_116_3: not s_116_2
        let s_116_3: Bits = !s_116_2;
        // C s_116_4: cast reint s_116_3 -> u64
        let s_116_4: u64 = (s_116_3.value() as u64);
        // D s_116_5: cast zx s_116_0 -> bv
        let s_116_5: Bits = Bits::new(s_116_0 as u128, 64u16);
        // C s_116_6: cast zx s_116_4 -> bv
        let s_116_6: Bits = Bits::new(s_116_4 as u128, 64u16);
        // D s_116_7: and s_116_5 s_116_6
        let s_116_7: Bits = ((s_116_5) & (s_116_6));
        // D s_116_8: cast reint s_116_7 -> u64
        let s_116_8: u64 = (s_116_7.value() as u64);
        // D s_116_9: call Mk_PMSNEVFR_EL1_Type(s_116_8)
        let s_116_9: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_116_8,
        );
        // D s_116_10: write-var tmp <= s_116_9
        fn_state.tmp = s_116_9;
        // N s_116_11: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_117_0: read-var tmp.0:struct
        let s_117_0: u64 = fn_state.tmp._0;
        // C s_117_1: const #64s : i
        let s_117_1: i128 = 64;
        // C s_117_2: const #576460752303423488u : u60
        let s_117_2: u64 = 576460752303423488;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 60u16);
        // D s_117_4: bits-cast zx s_117_3 -> bv length s_117_1
        let s_117_4: Bits = s_117_3.zero_extend(s_117_1);
        // D s_117_5: cast reint s_117_4 -> u64
        let s_117_5: u64 = (s_117_4.value() as u64);
        // D s_117_6: cast zx s_117_5 -> bv
        let s_117_6: Bits = Bits::new(s_117_5 as u128, 64u16);
        // D s_117_7: not s_117_6
        let s_117_7: Bits = !s_117_6;
        // D s_117_8: cast reint s_117_7 -> u64
        let s_117_8: u64 = (s_117_7.value() as u64);
        // D s_117_9: cast zx s_117_0 -> bv
        let s_117_9: Bits = Bits::new(s_117_0 as u128, 64u16);
        // D s_117_10: cast zx s_117_8 -> bv
        let s_117_10: Bits = Bits::new(s_117_8 as u128, 64u16);
        // D s_117_11: and s_117_9 s_117_10
        let s_117_11: Bits = ((s_117_9) & (s_117_10));
        // D s_117_12: cast reint s_117_11 -> u64
        let s_117_12: u64 = (s_117_11.value() as u64);
        // D s_117_13: call Mk_PMSNEVFR_EL1_Type(s_117_12)
        let s_117_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_117_12,
        );
        // D s_117_14: write-var tmp <= s_117_13
        fn_state.tmp = s_117_13;
        // N s_117_15: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_118_0: read-var tmp.0:struct
        let s_118_0: u64 = fn_state.tmp._0;
        // C s_118_1: const #64s : i
        let s_118_1: i128 = 64;
        // C s_118_2: const #288230376151711744u : u60
        let s_118_2: u64 = 288230376151711744;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 60u16);
        // D s_118_4: bits-cast zx s_118_3 -> bv length s_118_1
        let s_118_4: Bits = s_118_3.zero_extend(s_118_1);
        // D s_118_5: cast reint s_118_4 -> u64
        let s_118_5: u64 = (s_118_4.value() as u64);
        // D s_118_6: cast zx s_118_5 -> bv
        let s_118_6: Bits = Bits::new(s_118_5 as u128, 64u16);
        // D s_118_7: not s_118_6
        let s_118_7: Bits = !s_118_6;
        // D s_118_8: cast reint s_118_7 -> u64
        let s_118_8: u64 = (s_118_7.value() as u64);
        // D s_118_9: cast zx s_118_0 -> bv
        let s_118_9: Bits = Bits::new(s_118_0 as u128, 64u16);
        // D s_118_10: cast zx s_118_8 -> bv
        let s_118_10: Bits = Bits::new(s_118_8 as u128, 64u16);
        // D s_118_11: and s_118_9 s_118_10
        let s_118_11: Bits = ((s_118_9) & (s_118_10));
        // D s_118_12: cast reint s_118_11 -> u64
        let s_118_12: u64 = (s_118_11.value() as u64);
        // D s_118_13: call Mk_PMSNEVFR_EL1_Type(s_118_12)
        let s_118_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_118_12,
        );
        // D s_118_14: write-var tmp <= s_118_13
        fn_state.tmp = s_118_13;
        // N s_118_15: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_119_0: read-var tmp.0:struct
        let s_119_0: u64 = fn_state.tmp._0;
        // C s_119_1: const #64s : i
        let s_119_1: i128 = 64;
        // C s_119_2: const #144115188075855872u : u60
        let s_119_2: u64 = 144115188075855872;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 60u16);
        // D s_119_4: bits-cast zx s_119_3 -> bv length s_119_1
        let s_119_4: Bits = s_119_3.zero_extend(s_119_1);
        // D s_119_5: cast reint s_119_4 -> u64
        let s_119_5: u64 = (s_119_4.value() as u64);
        // D s_119_6: cast zx s_119_5 -> bv
        let s_119_6: Bits = Bits::new(s_119_5 as u128, 64u16);
        // D s_119_7: not s_119_6
        let s_119_7: Bits = !s_119_6;
        // D s_119_8: cast reint s_119_7 -> u64
        let s_119_8: u64 = (s_119_7.value() as u64);
        // D s_119_9: cast zx s_119_0 -> bv
        let s_119_9: Bits = Bits::new(s_119_0 as u128, 64u16);
        // D s_119_10: cast zx s_119_8 -> bv
        let s_119_10: Bits = Bits::new(s_119_8 as u128, 64u16);
        // D s_119_11: and s_119_9 s_119_10
        let s_119_11: Bits = ((s_119_9) & (s_119_10));
        // D s_119_12: cast reint s_119_11 -> u64
        let s_119_12: u64 = (s_119_11.value() as u64);
        // D s_119_13: call Mk_PMSNEVFR_EL1_Type(s_119_12)
        let s_119_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_119_12,
        );
        // D s_119_14: write-var tmp <= s_119_13
        fn_state.tmp = s_119_13;
        // N s_119_15: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_120_0: read-var tmp.0:struct
        let s_120_0: u64 = fn_state.tmp._0;
        // C s_120_1: const #64s : i
        let s_120_1: i128 = 64;
        // C s_120_2: const #72057594037927936u : u60
        let s_120_2: u64 = 72057594037927936;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 60u16);
        // D s_120_4: bits-cast zx s_120_3 -> bv length s_120_1
        let s_120_4: Bits = s_120_3.zero_extend(s_120_1);
        // D s_120_5: cast reint s_120_4 -> u64
        let s_120_5: u64 = (s_120_4.value() as u64);
        // D s_120_6: cast zx s_120_5 -> bv
        let s_120_6: Bits = Bits::new(s_120_5 as u128, 64u16);
        // D s_120_7: not s_120_6
        let s_120_7: Bits = !s_120_6;
        // D s_120_8: cast reint s_120_7 -> u64
        let s_120_8: u64 = (s_120_7.value() as u64);
        // D s_120_9: cast zx s_120_0 -> bv
        let s_120_9: Bits = Bits::new(s_120_0 as u128, 64u16);
        // D s_120_10: cast zx s_120_8 -> bv
        let s_120_10: Bits = Bits::new(s_120_8 as u128, 64u16);
        // D s_120_11: and s_120_9 s_120_10
        let s_120_11: Bits = ((s_120_9) & (s_120_10));
        // D s_120_12: cast reint s_120_11 -> u64
        let s_120_12: u64 = (s_120_11.value() as u64);
        // D s_120_13: call Mk_PMSNEVFR_EL1_Type(s_120_12)
        let s_120_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_120_12,
        );
        // D s_120_14: write-var tmp <= s_120_13
        fn_state.tmp = s_120_13;
        // N s_120_15: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_121_0: read-var tmp.0:struct
        let s_121_0: u64 = fn_state.tmp._0;
        // C s_121_1: const #64s : i
        let s_121_1: i128 = 64;
        // C s_121_2: const #36028797018963968u : u56
        let s_121_2: u64 = 36028797018963968;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 56u16);
        // D s_121_4: bits-cast zx s_121_3 -> bv length s_121_1
        let s_121_4: Bits = s_121_3.zero_extend(s_121_1);
        // D s_121_5: cast reint s_121_4 -> u64
        let s_121_5: u64 = (s_121_4.value() as u64);
        // D s_121_6: cast zx s_121_5 -> bv
        let s_121_6: Bits = Bits::new(s_121_5 as u128, 64u16);
        // D s_121_7: not s_121_6
        let s_121_7: Bits = !s_121_6;
        // D s_121_8: cast reint s_121_7 -> u64
        let s_121_8: u64 = (s_121_7.value() as u64);
        // D s_121_9: cast zx s_121_0 -> bv
        let s_121_9: Bits = Bits::new(s_121_0 as u128, 64u16);
        // D s_121_10: cast zx s_121_8 -> bv
        let s_121_10: Bits = Bits::new(s_121_8 as u128, 64u16);
        // D s_121_11: and s_121_9 s_121_10
        let s_121_11: Bits = ((s_121_9) & (s_121_10));
        // D s_121_12: cast reint s_121_11 -> u64
        let s_121_12: u64 = (s_121_11.value() as u64);
        // D s_121_13: call Mk_PMSNEVFR_EL1_Type(s_121_12)
        let s_121_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_121_12,
        );
        // D s_121_14: write-var tmp <= s_121_13
        fn_state.tmp = s_121_13;
        // N s_121_15: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_122_0: read-var tmp.0:struct
        let s_122_0: u64 = fn_state.tmp._0;
        // C s_122_1: const #64s : i
        let s_122_1: i128 = 64;
        // C s_122_2: const #18014398509481984u : u56
        let s_122_2: u64 = 18014398509481984;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 56u16);
        // D s_122_4: bits-cast zx s_122_3 -> bv length s_122_1
        let s_122_4: Bits = s_122_3.zero_extend(s_122_1);
        // D s_122_5: cast reint s_122_4 -> u64
        let s_122_5: u64 = (s_122_4.value() as u64);
        // D s_122_6: cast zx s_122_5 -> bv
        let s_122_6: Bits = Bits::new(s_122_5 as u128, 64u16);
        // D s_122_7: not s_122_6
        let s_122_7: Bits = !s_122_6;
        // D s_122_8: cast reint s_122_7 -> u64
        let s_122_8: u64 = (s_122_7.value() as u64);
        // D s_122_9: cast zx s_122_0 -> bv
        let s_122_9: Bits = Bits::new(s_122_0 as u128, 64u16);
        // D s_122_10: cast zx s_122_8 -> bv
        let s_122_10: Bits = Bits::new(s_122_8 as u128, 64u16);
        // D s_122_11: and s_122_9 s_122_10
        let s_122_11: Bits = ((s_122_9) & (s_122_10));
        // D s_122_12: cast reint s_122_11 -> u64
        let s_122_12: u64 = (s_122_11.value() as u64);
        // D s_122_13: call Mk_PMSNEVFR_EL1_Type(s_122_12)
        let s_122_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_122_12,
        );
        // D s_122_14: write-var tmp <= s_122_13
        fn_state.tmp = s_122_13;
        // N s_122_15: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_123_0: read-var tmp.0:struct
        let s_123_0: u64 = fn_state.tmp._0;
        // C s_123_1: const #64s : i
        let s_123_1: i128 = 64;
        // C s_123_2: const #9007199254740992u : u56
        let s_123_2: u64 = 9007199254740992;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 56u16);
        // D s_123_4: bits-cast zx s_123_3 -> bv length s_123_1
        let s_123_4: Bits = s_123_3.zero_extend(s_123_1);
        // D s_123_5: cast reint s_123_4 -> u64
        let s_123_5: u64 = (s_123_4.value() as u64);
        // D s_123_6: cast zx s_123_5 -> bv
        let s_123_6: Bits = Bits::new(s_123_5 as u128, 64u16);
        // D s_123_7: not s_123_6
        let s_123_7: Bits = !s_123_6;
        // D s_123_8: cast reint s_123_7 -> u64
        let s_123_8: u64 = (s_123_7.value() as u64);
        // D s_123_9: cast zx s_123_0 -> bv
        let s_123_9: Bits = Bits::new(s_123_0 as u128, 64u16);
        // D s_123_10: cast zx s_123_8 -> bv
        let s_123_10: Bits = Bits::new(s_123_8 as u128, 64u16);
        // D s_123_11: and s_123_9 s_123_10
        let s_123_11: Bits = ((s_123_9) & (s_123_10));
        // D s_123_12: cast reint s_123_11 -> u64
        let s_123_12: u64 = (s_123_11.value() as u64);
        // D s_123_13: call Mk_PMSNEVFR_EL1_Type(s_123_12)
        let s_123_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_123_12,
        );
        // D s_123_14: write-var tmp <= s_123_13
        fn_state.tmp = s_123_13;
        // N s_123_15: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_124_0: read-var tmp.0:struct
        let s_124_0: u64 = fn_state.tmp._0;
        // C s_124_1: const #64s : i
        let s_124_1: i128 = 64;
        // C s_124_2: const #4503599627370496u : u56
        let s_124_2: u64 = 4503599627370496;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 56u16);
        // D s_124_4: bits-cast zx s_124_3 -> bv length s_124_1
        let s_124_4: Bits = s_124_3.zero_extend(s_124_1);
        // D s_124_5: cast reint s_124_4 -> u64
        let s_124_5: u64 = (s_124_4.value() as u64);
        // D s_124_6: cast zx s_124_5 -> bv
        let s_124_6: Bits = Bits::new(s_124_5 as u128, 64u16);
        // D s_124_7: not s_124_6
        let s_124_7: Bits = !s_124_6;
        // D s_124_8: cast reint s_124_7 -> u64
        let s_124_8: u64 = (s_124_7.value() as u64);
        // D s_124_9: cast zx s_124_0 -> bv
        let s_124_9: Bits = Bits::new(s_124_0 as u128, 64u16);
        // D s_124_10: cast zx s_124_8 -> bv
        let s_124_10: Bits = Bits::new(s_124_8 as u128, 64u16);
        // D s_124_11: and s_124_9 s_124_10
        let s_124_11: Bits = ((s_124_9) & (s_124_10));
        // D s_124_12: cast reint s_124_11 -> u64
        let s_124_12: u64 = (s_124_11.value() as u64);
        // D s_124_13: call Mk_PMSNEVFR_EL1_Type(s_124_12)
        let s_124_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_124_12,
        );
        // D s_124_14: write-var tmp <= s_124_13
        fn_state.tmp = s_124_13;
        // N s_124_15: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_125_0: read-var tmp.0:struct
        let s_125_0: u64 = fn_state.tmp._0;
        // C s_125_1: const #64s : i
        let s_125_1: i128 = 64;
        // C s_125_2: const #2251799813685248u : u52
        let s_125_2: u64 = 2251799813685248;
        // C s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 52u16);
        // D s_125_4: bits-cast zx s_125_3 -> bv length s_125_1
        let s_125_4: Bits = s_125_3.zero_extend(s_125_1);
        // D s_125_5: cast reint s_125_4 -> u64
        let s_125_5: u64 = (s_125_4.value() as u64);
        // D s_125_6: cast zx s_125_5 -> bv
        let s_125_6: Bits = Bits::new(s_125_5 as u128, 64u16);
        // D s_125_7: not s_125_6
        let s_125_7: Bits = !s_125_6;
        // D s_125_8: cast reint s_125_7 -> u64
        let s_125_8: u64 = (s_125_7.value() as u64);
        // D s_125_9: cast zx s_125_0 -> bv
        let s_125_9: Bits = Bits::new(s_125_0 as u128, 64u16);
        // D s_125_10: cast zx s_125_8 -> bv
        let s_125_10: Bits = Bits::new(s_125_8 as u128, 64u16);
        // D s_125_11: and s_125_9 s_125_10
        let s_125_11: Bits = ((s_125_9) & (s_125_10));
        // D s_125_12: cast reint s_125_11 -> u64
        let s_125_12: u64 = (s_125_11.value() as u64);
        // D s_125_13: call Mk_PMSNEVFR_EL1_Type(s_125_12)
        let s_125_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_125_12,
        );
        // D s_125_14: write-var tmp <= s_125_13
        fn_state.tmp = s_125_13;
        // N s_125_15: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_126_0: read-var tmp.0:struct
        let s_126_0: u64 = fn_state.tmp._0;
        // C s_126_1: const #64s : i
        let s_126_1: i128 = 64;
        // C s_126_2: const #1125899906842624u : u52
        let s_126_2: u64 = 1125899906842624;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 52u16);
        // D s_126_4: bits-cast zx s_126_3 -> bv length s_126_1
        let s_126_4: Bits = s_126_3.zero_extend(s_126_1);
        // D s_126_5: cast reint s_126_4 -> u64
        let s_126_5: u64 = (s_126_4.value() as u64);
        // D s_126_6: cast zx s_126_5 -> bv
        let s_126_6: Bits = Bits::new(s_126_5 as u128, 64u16);
        // D s_126_7: not s_126_6
        let s_126_7: Bits = !s_126_6;
        // D s_126_8: cast reint s_126_7 -> u64
        let s_126_8: u64 = (s_126_7.value() as u64);
        // D s_126_9: cast zx s_126_0 -> bv
        let s_126_9: Bits = Bits::new(s_126_0 as u128, 64u16);
        // D s_126_10: cast zx s_126_8 -> bv
        let s_126_10: Bits = Bits::new(s_126_8 as u128, 64u16);
        // D s_126_11: and s_126_9 s_126_10
        let s_126_11: Bits = ((s_126_9) & (s_126_10));
        // D s_126_12: cast reint s_126_11 -> u64
        let s_126_12: u64 = (s_126_11.value() as u64);
        // D s_126_13: call Mk_PMSNEVFR_EL1_Type(s_126_12)
        let s_126_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_126_12,
        );
        // D s_126_14: write-var tmp <= s_126_13
        fn_state.tmp = s_126_13;
        // N s_126_15: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_127_0: read-var tmp.0:struct
        let s_127_0: u64 = fn_state.tmp._0;
        // C s_127_1: const #64s : i
        let s_127_1: i128 = 64;
        // C s_127_2: const #562949953421312u : u52
        let s_127_2: u64 = 562949953421312;
        // C s_127_3: cast zx s_127_2 -> bv
        let s_127_3: Bits = Bits::new(s_127_2 as u128, 52u16);
        // D s_127_4: bits-cast zx s_127_3 -> bv length s_127_1
        let s_127_4: Bits = s_127_3.zero_extend(s_127_1);
        // D s_127_5: cast reint s_127_4 -> u64
        let s_127_5: u64 = (s_127_4.value() as u64);
        // D s_127_6: cast zx s_127_5 -> bv
        let s_127_6: Bits = Bits::new(s_127_5 as u128, 64u16);
        // D s_127_7: not s_127_6
        let s_127_7: Bits = !s_127_6;
        // D s_127_8: cast reint s_127_7 -> u64
        let s_127_8: u64 = (s_127_7.value() as u64);
        // D s_127_9: cast zx s_127_0 -> bv
        let s_127_9: Bits = Bits::new(s_127_0 as u128, 64u16);
        // D s_127_10: cast zx s_127_8 -> bv
        let s_127_10: Bits = Bits::new(s_127_8 as u128, 64u16);
        // D s_127_11: and s_127_9 s_127_10
        let s_127_11: Bits = ((s_127_9) & (s_127_10));
        // D s_127_12: cast reint s_127_11 -> u64
        let s_127_12: u64 = (s_127_11.value() as u64);
        // D s_127_13: call Mk_PMSNEVFR_EL1_Type(s_127_12)
        let s_127_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_127_12,
        );
        // D s_127_14: write-var tmp <= s_127_13
        fn_state.tmp = s_127_13;
        // N s_127_15: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_128_0: read-var tmp.0:struct
        let s_128_0: u64 = fn_state.tmp._0;
        // C s_128_1: const #64s : i
        let s_128_1: i128 = 64;
        // C s_128_2: const #281474976710656u : u52
        let s_128_2: u64 = 281474976710656;
        // C s_128_3: cast zx s_128_2 -> bv
        let s_128_3: Bits = Bits::new(s_128_2 as u128, 52u16);
        // D s_128_4: bits-cast zx s_128_3 -> bv length s_128_1
        let s_128_4: Bits = s_128_3.zero_extend(s_128_1);
        // D s_128_5: cast reint s_128_4 -> u64
        let s_128_5: u64 = (s_128_4.value() as u64);
        // D s_128_6: cast zx s_128_5 -> bv
        let s_128_6: Bits = Bits::new(s_128_5 as u128, 64u16);
        // D s_128_7: not s_128_6
        let s_128_7: Bits = !s_128_6;
        // D s_128_8: cast reint s_128_7 -> u64
        let s_128_8: u64 = (s_128_7.value() as u64);
        // D s_128_9: cast zx s_128_0 -> bv
        let s_128_9: Bits = Bits::new(s_128_0 as u128, 64u16);
        // D s_128_10: cast zx s_128_8 -> bv
        let s_128_10: Bits = Bits::new(s_128_8 as u128, 64u16);
        // D s_128_11: and s_128_9 s_128_10
        let s_128_11: Bits = ((s_128_9) & (s_128_10));
        // D s_128_12: cast reint s_128_11 -> u64
        let s_128_12: u64 = (s_128_11.value() as u64);
        // D s_128_13: call Mk_PMSNEVFR_EL1_Type(s_128_12)
        let s_128_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_128_12,
        );
        // D s_128_14: write-var tmp <= s_128_13
        fn_state.tmp = s_128_13;
        // N s_128_15: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_129_0: read-var tmp.0:struct
        let s_129_0: u64 = fn_state.tmp._0;
        // C s_129_1: const #64s : i
        let s_129_1: i128 = 64;
        // C s_129_2: const #2147483648u : u32
        let s_129_2: u32 = 2147483648;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 32u16);
        // D s_129_4: bits-cast zx s_129_3 -> bv length s_129_1
        let s_129_4: Bits = s_129_3.zero_extend(s_129_1);
        // D s_129_5: cast reint s_129_4 -> u64
        let s_129_5: u64 = (s_129_4.value() as u64);
        // D s_129_6: cast zx s_129_5 -> bv
        let s_129_6: Bits = Bits::new(s_129_5 as u128, 64u16);
        // D s_129_7: not s_129_6
        let s_129_7: Bits = !s_129_6;
        // D s_129_8: cast reint s_129_7 -> u64
        let s_129_8: u64 = (s_129_7.value() as u64);
        // D s_129_9: cast zx s_129_0 -> bv
        let s_129_9: Bits = Bits::new(s_129_0 as u128, 64u16);
        // D s_129_10: cast zx s_129_8 -> bv
        let s_129_10: Bits = Bits::new(s_129_8 as u128, 64u16);
        // D s_129_11: and s_129_9 s_129_10
        let s_129_11: Bits = ((s_129_9) & (s_129_10));
        // D s_129_12: cast reint s_129_11 -> u64
        let s_129_12: u64 = (s_129_11.value() as u64);
        // D s_129_13: call Mk_PMSNEVFR_EL1_Type(s_129_12)
        let s_129_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_129_12,
        );
        // D s_129_14: write-var tmp <= s_129_13
        fn_state.tmp = s_129_13;
        // N s_129_15: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_130_0: read-var tmp.0:struct
        let s_130_0: u64 = fn_state.tmp._0;
        // C s_130_1: const #64s : i
        let s_130_1: i128 = 64;
        // C s_130_2: const #1073741824u : u32
        let s_130_2: u32 = 1073741824;
        // C s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 32u16);
        // D s_130_4: bits-cast zx s_130_3 -> bv length s_130_1
        let s_130_4: Bits = s_130_3.zero_extend(s_130_1);
        // D s_130_5: cast reint s_130_4 -> u64
        let s_130_5: u64 = (s_130_4.value() as u64);
        // D s_130_6: cast zx s_130_5 -> bv
        let s_130_6: Bits = Bits::new(s_130_5 as u128, 64u16);
        // D s_130_7: not s_130_6
        let s_130_7: Bits = !s_130_6;
        // D s_130_8: cast reint s_130_7 -> u64
        let s_130_8: u64 = (s_130_7.value() as u64);
        // D s_130_9: cast zx s_130_0 -> bv
        let s_130_9: Bits = Bits::new(s_130_0 as u128, 64u16);
        // D s_130_10: cast zx s_130_8 -> bv
        let s_130_10: Bits = Bits::new(s_130_8 as u128, 64u16);
        // D s_130_11: and s_130_9 s_130_10
        let s_130_11: Bits = ((s_130_9) & (s_130_10));
        // D s_130_12: cast reint s_130_11 -> u64
        let s_130_12: u64 = (s_130_11.value() as u64);
        // D s_130_13: call Mk_PMSNEVFR_EL1_Type(s_130_12)
        let s_130_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_130_12,
        );
        // D s_130_14: write-var tmp <= s_130_13
        fn_state.tmp = s_130_13;
        // N s_130_15: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_131_0: read-var tmp.0:struct
        let s_131_0: u64 = fn_state.tmp._0;
        // C s_131_1: const #64s : i
        let s_131_1: i128 = 64;
        // C s_131_2: const #536870912u : u32
        let s_131_2: u32 = 536870912;
        // C s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 32u16);
        // D s_131_4: bits-cast zx s_131_3 -> bv length s_131_1
        let s_131_4: Bits = s_131_3.zero_extend(s_131_1);
        // D s_131_5: cast reint s_131_4 -> u64
        let s_131_5: u64 = (s_131_4.value() as u64);
        // D s_131_6: cast zx s_131_5 -> bv
        let s_131_6: Bits = Bits::new(s_131_5 as u128, 64u16);
        // D s_131_7: not s_131_6
        let s_131_7: Bits = !s_131_6;
        // D s_131_8: cast reint s_131_7 -> u64
        let s_131_8: u64 = (s_131_7.value() as u64);
        // D s_131_9: cast zx s_131_0 -> bv
        let s_131_9: Bits = Bits::new(s_131_0 as u128, 64u16);
        // D s_131_10: cast zx s_131_8 -> bv
        let s_131_10: Bits = Bits::new(s_131_8 as u128, 64u16);
        // D s_131_11: and s_131_9 s_131_10
        let s_131_11: Bits = ((s_131_9) & (s_131_10));
        // D s_131_12: cast reint s_131_11 -> u64
        let s_131_12: u64 = (s_131_11.value() as u64);
        // D s_131_13: call Mk_PMSNEVFR_EL1_Type(s_131_12)
        let s_131_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_131_12,
        );
        // D s_131_14: write-var tmp <= s_131_13
        fn_state.tmp = s_131_13;
        // N s_131_15: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_132_0: read-var tmp.0:struct
        let s_132_0: u64 = fn_state.tmp._0;
        // C s_132_1: const #64s : i
        let s_132_1: i128 = 64;
        // C s_132_2: const #268435456u : u32
        let s_132_2: u32 = 268435456;
        // C s_132_3: cast zx s_132_2 -> bv
        let s_132_3: Bits = Bits::new(s_132_2 as u128, 32u16);
        // D s_132_4: bits-cast zx s_132_3 -> bv length s_132_1
        let s_132_4: Bits = s_132_3.zero_extend(s_132_1);
        // D s_132_5: cast reint s_132_4 -> u64
        let s_132_5: u64 = (s_132_4.value() as u64);
        // D s_132_6: cast zx s_132_5 -> bv
        let s_132_6: Bits = Bits::new(s_132_5 as u128, 64u16);
        // D s_132_7: not s_132_6
        let s_132_7: Bits = !s_132_6;
        // D s_132_8: cast reint s_132_7 -> u64
        let s_132_8: u64 = (s_132_7.value() as u64);
        // D s_132_9: cast zx s_132_0 -> bv
        let s_132_9: Bits = Bits::new(s_132_0 as u128, 64u16);
        // D s_132_10: cast zx s_132_8 -> bv
        let s_132_10: Bits = Bits::new(s_132_8 as u128, 64u16);
        // D s_132_11: and s_132_9 s_132_10
        let s_132_11: Bits = ((s_132_9) & (s_132_10));
        // D s_132_12: cast reint s_132_11 -> u64
        let s_132_12: u64 = (s_132_11.value() as u64);
        // D s_132_13: call Mk_PMSNEVFR_EL1_Type(s_132_12)
        let s_132_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_132_12,
        );
        // D s_132_14: write-var tmp <= s_132_13
        fn_state.tmp = s_132_13;
        // N s_132_15: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_133_0: read-var tmp.0:struct
        let s_133_0: u64 = fn_state.tmp._0;
        // C s_133_1: const #64s : i
        let s_133_1: i128 = 64;
        // C s_133_2: const #134217728u : u28
        let s_133_2: u32 = 134217728;
        // C s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 28u16);
        // D s_133_4: bits-cast zx s_133_3 -> bv length s_133_1
        let s_133_4: Bits = s_133_3.zero_extend(s_133_1);
        // D s_133_5: cast reint s_133_4 -> u64
        let s_133_5: u64 = (s_133_4.value() as u64);
        // D s_133_6: cast zx s_133_5 -> bv
        let s_133_6: Bits = Bits::new(s_133_5 as u128, 64u16);
        // D s_133_7: not s_133_6
        let s_133_7: Bits = !s_133_6;
        // D s_133_8: cast reint s_133_7 -> u64
        let s_133_8: u64 = (s_133_7.value() as u64);
        // D s_133_9: cast zx s_133_0 -> bv
        let s_133_9: Bits = Bits::new(s_133_0 as u128, 64u16);
        // D s_133_10: cast zx s_133_8 -> bv
        let s_133_10: Bits = Bits::new(s_133_8 as u128, 64u16);
        // D s_133_11: and s_133_9 s_133_10
        let s_133_11: Bits = ((s_133_9) & (s_133_10));
        // D s_133_12: cast reint s_133_11 -> u64
        let s_133_12: u64 = (s_133_11.value() as u64);
        // D s_133_13: call Mk_PMSNEVFR_EL1_Type(s_133_12)
        let s_133_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_133_12,
        );
        // D s_133_14: write-var tmp <= s_133_13
        fn_state.tmp = s_133_13;
        // N s_133_15: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_134_0: read-var tmp.0:struct
        let s_134_0: u64 = fn_state.tmp._0;
        // C s_134_1: const #64s : i
        let s_134_1: i128 = 64;
        // C s_134_2: const #67108864u : u28
        let s_134_2: u32 = 67108864;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 28u16);
        // D s_134_4: bits-cast zx s_134_3 -> bv length s_134_1
        let s_134_4: Bits = s_134_3.zero_extend(s_134_1);
        // D s_134_5: cast reint s_134_4 -> u64
        let s_134_5: u64 = (s_134_4.value() as u64);
        // D s_134_6: cast zx s_134_5 -> bv
        let s_134_6: Bits = Bits::new(s_134_5 as u128, 64u16);
        // D s_134_7: not s_134_6
        let s_134_7: Bits = !s_134_6;
        // D s_134_8: cast reint s_134_7 -> u64
        let s_134_8: u64 = (s_134_7.value() as u64);
        // D s_134_9: cast zx s_134_0 -> bv
        let s_134_9: Bits = Bits::new(s_134_0 as u128, 64u16);
        // D s_134_10: cast zx s_134_8 -> bv
        let s_134_10: Bits = Bits::new(s_134_8 as u128, 64u16);
        // D s_134_11: and s_134_9 s_134_10
        let s_134_11: Bits = ((s_134_9) & (s_134_10));
        // D s_134_12: cast reint s_134_11 -> u64
        let s_134_12: u64 = (s_134_11.value() as u64);
        // D s_134_13: call Mk_PMSNEVFR_EL1_Type(s_134_12)
        let s_134_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_134_12,
        );
        // D s_134_14: write-var tmp <= s_134_13
        fn_state.tmp = s_134_13;
        // N s_134_15: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_135_0: read-var tmp.0:struct
        let s_135_0: u64 = fn_state.tmp._0;
        // C s_135_1: const #64s : i
        let s_135_1: i128 = 64;
        // C s_135_2: const #33554432u : u28
        let s_135_2: u32 = 33554432;
        // C s_135_3: cast zx s_135_2 -> bv
        let s_135_3: Bits = Bits::new(s_135_2 as u128, 28u16);
        // D s_135_4: bits-cast zx s_135_3 -> bv length s_135_1
        let s_135_4: Bits = s_135_3.zero_extend(s_135_1);
        // D s_135_5: cast reint s_135_4 -> u64
        let s_135_5: u64 = (s_135_4.value() as u64);
        // D s_135_6: cast zx s_135_5 -> bv
        let s_135_6: Bits = Bits::new(s_135_5 as u128, 64u16);
        // D s_135_7: not s_135_6
        let s_135_7: Bits = !s_135_6;
        // D s_135_8: cast reint s_135_7 -> u64
        let s_135_8: u64 = (s_135_7.value() as u64);
        // D s_135_9: cast zx s_135_0 -> bv
        let s_135_9: Bits = Bits::new(s_135_0 as u128, 64u16);
        // D s_135_10: cast zx s_135_8 -> bv
        let s_135_10: Bits = Bits::new(s_135_8 as u128, 64u16);
        // D s_135_11: and s_135_9 s_135_10
        let s_135_11: Bits = ((s_135_9) & (s_135_10));
        // D s_135_12: cast reint s_135_11 -> u64
        let s_135_12: u64 = (s_135_11.value() as u64);
        // D s_135_13: call Mk_PMSNEVFR_EL1_Type(s_135_12)
        let s_135_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_135_12,
        );
        // D s_135_14: write-var tmp <= s_135_13
        fn_state.tmp = s_135_13;
        // N s_135_15: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_136_0: read-var tmp.0:struct
        let s_136_0: u64 = fn_state.tmp._0;
        // C s_136_1: const #64s : i
        let s_136_1: i128 = 64;
        // C s_136_2: const #16777216u : u28
        let s_136_2: u32 = 16777216;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 28u16);
        // D s_136_4: bits-cast zx s_136_3 -> bv length s_136_1
        let s_136_4: Bits = s_136_3.zero_extend(s_136_1);
        // D s_136_5: cast reint s_136_4 -> u64
        let s_136_5: u64 = (s_136_4.value() as u64);
        // D s_136_6: cast zx s_136_5 -> bv
        let s_136_6: Bits = Bits::new(s_136_5 as u128, 64u16);
        // D s_136_7: not s_136_6
        let s_136_7: Bits = !s_136_6;
        // D s_136_8: cast reint s_136_7 -> u64
        let s_136_8: u64 = (s_136_7.value() as u64);
        // D s_136_9: cast zx s_136_0 -> bv
        let s_136_9: Bits = Bits::new(s_136_0 as u128, 64u16);
        // D s_136_10: cast zx s_136_8 -> bv
        let s_136_10: Bits = Bits::new(s_136_8 as u128, 64u16);
        // D s_136_11: and s_136_9 s_136_10
        let s_136_11: Bits = ((s_136_9) & (s_136_10));
        // D s_136_12: cast reint s_136_11 -> u64
        let s_136_12: u64 = (s_136_11.value() as u64);
        // D s_136_13: call Mk_PMSNEVFR_EL1_Type(s_136_12)
        let s_136_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_136_12,
        );
        // D s_136_14: write-var tmp <= s_136_13
        fn_state.tmp = s_136_13;
        // N s_136_15: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_137_0: read-var tmp.0:struct
        let s_137_0: u64 = fn_state.tmp._0;
        // C s_137_1: const #64s : i
        let s_137_1: i128 = 64;
        // C s_137_2: const #8388608u : u24
        let s_137_2: u32 = 8388608;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 24u16);
        // D s_137_4: bits-cast zx s_137_3 -> bv length s_137_1
        let s_137_4: Bits = s_137_3.zero_extend(s_137_1);
        // D s_137_5: cast reint s_137_4 -> u64
        let s_137_5: u64 = (s_137_4.value() as u64);
        // D s_137_6: cast zx s_137_5 -> bv
        let s_137_6: Bits = Bits::new(s_137_5 as u128, 64u16);
        // D s_137_7: not s_137_6
        let s_137_7: Bits = !s_137_6;
        // D s_137_8: cast reint s_137_7 -> u64
        let s_137_8: u64 = (s_137_7.value() as u64);
        // D s_137_9: cast zx s_137_0 -> bv
        let s_137_9: Bits = Bits::new(s_137_0 as u128, 64u16);
        // D s_137_10: cast zx s_137_8 -> bv
        let s_137_10: Bits = Bits::new(s_137_8 as u128, 64u16);
        // D s_137_11: and s_137_9 s_137_10
        let s_137_11: Bits = ((s_137_9) & (s_137_10));
        // D s_137_12: cast reint s_137_11 -> u64
        let s_137_12: u64 = (s_137_11.value() as u64);
        // D s_137_13: call Mk_PMSNEVFR_EL1_Type(s_137_12)
        let s_137_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_137_12,
        );
        // D s_137_14: write-var tmp <= s_137_13
        fn_state.tmp = s_137_13;
        // N s_137_15: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_138_0: const #"event 23 is implemented" : str
        let s_138_0: &'static str = "event 23 is implemented";
        // S s_138_1: call __IMPDEF_boolean(s_138_0)
        let s_138_1: bool = u__IMPDEF_boolean(state, tracer, s_138_0);
        // D s_138_2: write-var gs#37701 <= s_138_1
        fn_state.gs_37701 = s_138_1;
        // N s_138_3: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_139_0: read-var tmp.0:struct
        let s_139_0: u64 = fn_state.tmp._0;
        // C s_139_1: const #64s : i
        let s_139_1: i128 = 64;
        // C s_139_2: const #4194304u : u24
        let s_139_2: u32 = 4194304;
        // C s_139_3: cast zx s_139_2 -> bv
        let s_139_3: Bits = Bits::new(s_139_2 as u128, 24u16);
        // D s_139_4: bits-cast zx s_139_3 -> bv length s_139_1
        let s_139_4: Bits = s_139_3.zero_extend(s_139_1);
        // D s_139_5: cast reint s_139_4 -> u64
        let s_139_5: u64 = (s_139_4.value() as u64);
        // D s_139_6: cast zx s_139_5 -> bv
        let s_139_6: Bits = Bits::new(s_139_5 as u128, 64u16);
        // D s_139_7: not s_139_6
        let s_139_7: Bits = !s_139_6;
        // D s_139_8: cast reint s_139_7 -> u64
        let s_139_8: u64 = (s_139_7.value() as u64);
        // D s_139_9: cast zx s_139_0 -> bv
        let s_139_9: Bits = Bits::new(s_139_0 as u128, 64u16);
        // D s_139_10: cast zx s_139_8 -> bv
        let s_139_10: Bits = Bits::new(s_139_8 as u128, 64u16);
        // D s_139_11: and s_139_9 s_139_10
        let s_139_11: Bits = ((s_139_9) & (s_139_10));
        // D s_139_12: cast reint s_139_11 -> u64
        let s_139_12: u64 = (s_139_11.value() as u64);
        // D s_139_13: call Mk_PMSNEVFR_EL1_Type(s_139_12)
        let s_139_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_139_12,
        );
        // D s_139_14: write-var tmp <= s_139_13
        fn_state.tmp = s_139_13;
        // N s_139_15: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_140_0: const #"event 22 is implemented" : str
        let s_140_0: &'static str = "event 22 is implemented";
        // S s_140_1: call __IMPDEF_boolean(s_140_0)
        let s_140_1: bool = u__IMPDEF_boolean(state, tracer, s_140_0);
        // D s_140_2: write-var gs#37700 <= s_140_1
        fn_state.gs_37700 = s_140_1;
        // N s_140_3: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_141_0: read-var tmp.0:struct
        let s_141_0: u64 = fn_state.tmp._0;
        // C s_141_1: const #64s : i
        let s_141_1: i128 = 64;
        // C s_141_2: const #2097152u : u24
        let s_141_2: u32 = 2097152;
        // C s_141_3: cast zx s_141_2 -> bv
        let s_141_3: Bits = Bits::new(s_141_2 as u128, 24u16);
        // D s_141_4: bits-cast zx s_141_3 -> bv length s_141_1
        let s_141_4: Bits = s_141_3.zero_extend(s_141_1);
        // D s_141_5: cast reint s_141_4 -> u64
        let s_141_5: u64 = (s_141_4.value() as u64);
        // D s_141_6: cast zx s_141_5 -> bv
        let s_141_6: Bits = Bits::new(s_141_5 as u128, 64u16);
        // D s_141_7: not s_141_6
        let s_141_7: Bits = !s_141_6;
        // D s_141_8: cast reint s_141_7 -> u64
        let s_141_8: u64 = (s_141_7.value() as u64);
        // D s_141_9: cast zx s_141_0 -> bv
        let s_141_9: Bits = Bits::new(s_141_0 as u128, 64u16);
        // D s_141_10: cast zx s_141_8 -> bv
        let s_141_10: Bits = Bits::new(s_141_8 as u128, 64u16);
        // D s_141_11: and s_141_9 s_141_10
        let s_141_11: Bits = ((s_141_9) & (s_141_10));
        // D s_141_12: cast reint s_141_11 -> u64
        let s_141_12: u64 = (s_141_11.value() as u64);
        // D s_141_13: call Mk_PMSNEVFR_EL1_Type(s_141_12)
        let s_141_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_141_12,
        );
        // D s_141_14: write-var tmp <= s_141_13
        fn_state.tmp = s_141_13;
        // N s_141_15: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_142_0: const #"event 21 is implemented" : str
        let s_142_0: &'static str = "event 21 is implemented";
        // S s_142_1: call __IMPDEF_boolean(s_142_0)
        let s_142_1: bool = u__IMPDEF_boolean(state, tracer, s_142_0);
        // D s_142_2: write-var gs#37699 <= s_142_1
        fn_state.gs_37699 = s_142_1;
        // N s_142_3: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_143_0: read-var tmp.0:struct
        let s_143_0: u64 = fn_state.tmp._0;
        // C s_143_1: const #64s : i
        let s_143_1: i128 = 64;
        // C s_143_2: const #1048576u : u24
        let s_143_2: u32 = 1048576;
        // C s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 24u16);
        // D s_143_4: bits-cast zx s_143_3 -> bv length s_143_1
        let s_143_4: Bits = s_143_3.zero_extend(s_143_1);
        // D s_143_5: cast reint s_143_4 -> u64
        let s_143_5: u64 = (s_143_4.value() as u64);
        // D s_143_6: cast zx s_143_5 -> bv
        let s_143_6: Bits = Bits::new(s_143_5 as u128, 64u16);
        // D s_143_7: not s_143_6
        let s_143_7: Bits = !s_143_6;
        // D s_143_8: cast reint s_143_7 -> u64
        let s_143_8: u64 = (s_143_7.value() as u64);
        // D s_143_9: cast zx s_143_0 -> bv
        let s_143_9: Bits = Bits::new(s_143_0 as u128, 64u16);
        // D s_143_10: cast zx s_143_8 -> bv
        let s_143_10: Bits = Bits::new(s_143_8 as u128, 64u16);
        // D s_143_11: and s_143_9 s_143_10
        let s_143_11: Bits = ((s_143_9) & (s_143_10));
        // D s_143_12: cast reint s_143_11 -> u64
        let s_143_12: u64 = (s_143_11.value() as u64);
        // D s_143_13: call Mk_PMSNEVFR_EL1_Type(s_143_12)
        let s_143_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_143_12,
        );
        // D s_143_14: write-var tmp <= s_143_13
        fn_state.tmp = s_143_13;
        // N s_143_15: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_144_0: const #"event 20 is implemented" : str
        let s_144_0: &'static str = "event 20 is implemented";
        // S s_144_1: call __IMPDEF_boolean(s_144_0)
        let s_144_1: bool = u__IMPDEF_boolean(state, tracer, s_144_0);
        // D s_144_2: write-var gs#37698 <= s_144_1
        fn_state.gs_37698 = s_144_1;
        // N s_144_3: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_145_0: read-var tmp.0:struct
        let s_145_0: u64 = fn_state.tmp._0;
        // C s_145_1: const #64s : i
        let s_145_1: i128 = 64;
        // C s_145_2: const #524288u : u20
        let s_145_2: u32 = 524288;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 20u16);
        // D s_145_4: bits-cast zx s_145_3 -> bv length s_145_1
        let s_145_4: Bits = s_145_3.zero_extend(s_145_1);
        // D s_145_5: cast reint s_145_4 -> u64
        let s_145_5: u64 = (s_145_4.value() as u64);
        // D s_145_6: cast zx s_145_5 -> bv
        let s_145_6: Bits = Bits::new(s_145_5 as u128, 64u16);
        // D s_145_7: not s_145_6
        let s_145_7: Bits = !s_145_6;
        // D s_145_8: cast reint s_145_7 -> u64
        let s_145_8: u64 = (s_145_7.value() as u64);
        // D s_145_9: cast zx s_145_0 -> bv
        let s_145_9: Bits = Bits::new(s_145_0 as u128, 64u16);
        // D s_145_10: cast zx s_145_8 -> bv
        let s_145_10: Bits = Bits::new(s_145_8 as u128, 64u16);
        // D s_145_11: and s_145_9 s_145_10
        let s_145_11: Bits = ((s_145_9) & (s_145_10));
        // D s_145_12: cast reint s_145_11 -> u64
        let s_145_12: u64 = (s_145_11.value() as u64);
        // D s_145_13: call Mk_PMSNEVFR_EL1_Type(s_145_12)
        let s_145_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_145_12,
        );
        // D s_145_14: write-var tmp <= s_145_13
        fn_state.tmp = s_145_13;
        // N s_145_15: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_146_0: const #"event 19 is implemented" : str
        let s_146_0: &'static str = "event 19 is implemented";
        // S s_146_1: call __IMPDEF_boolean(s_146_0)
        let s_146_1: bool = u__IMPDEF_boolean(state, tracer, s_146_0);
        // D s_146_2: write-var gs#37697 <= s_146_1
        fn_state.gs_37697 = s_146_1;
        // N s_146_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_147_0: read-var tmp.0:struct
        let s_147_0: u64 = fn_state.tmp._0;
        // C s_147_1: const #64s : i
        let s_147_1: i128 = 64;
        // C s_147_2: const #393216u : u20
        let s_147_2: u32 = 393216;
        // C s_147_3: cast zx s_147_2 -> bv
        let s_147_3: Bits = Bits::new(s_147_2 as u128, 20u16);
        // D s_147_4: bits-cast zx s_147_3 -> bv length s_147_1
        let s_147_4: Bits = s_147_3.zero_extend(s_147_1);
        // D s_147_5: cast reint s_147_4 -> u64
        let s_147_5: u64 = (s_147_4.value() as u64);
        // D s_147_6: cast zx s_147_5 -> bv
        let s_147_6: Bits = Bits::new(s_147_5 as u128, 64u16);
        // D s_147_7: not s_147_6
        let s_147_7: Bits = !s_147_6;
        // D s_147_8: cast reint s_147_7 -> u64
        let s_147_8: u64 = (s_147_7.value() as u64);
        // D s_147_9: cast zx s_147_0 -> bv
        let s_147_9: Bits = Bits::new(s_147_0 as u128, 64u16);
        // D s_147_10: cast zx s_147_8 -> bv
        let s_147_10: Bits = Bits::new(s_147_8 as u128, 64u16);
        // D s_147_11: and s_147_9 s_147_10
        let s_147_11: Bits = ((s_147_9) & (s_147_10));
        // D s_147_12: cast reint s_147_11 -> u64
        let s_147_12: u64 = (s_147_11.value() as u64);
        // D s_147_13: call Mk_PMSNEVFR_EL1_Type(s_147_12)
        let s_147_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_147_12,
        );
        // D s_147_14: write-var tmp <= s_147_13
        fn_state.tmp = s_147_13;
        // N s_147_15: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_148_0: const #99u : u32
        let s_148_0: u32 = 99;
        // S s_148_1: call IsFeatureImplemented(s_148_0)
        let s_148_1: bool = IsFeatureImplemented(state, tracer, s_148_0);
        // D s_148_2: write-var gs#37696 <= s_148_1
        fn_state.gs_37696 = s_148_1;
        // N s_148_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_149_0: read-var tmp.0:struct
        let s_149_0: u64 = fn_state.tmp._0;
        // C s_149_1: const #64s : i
        let s_149_1: i128 = 64;
        // C s_149_2: const #65536u : u20
        let s_149_2: u32 = 65536;
        // C s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 20u16);
        // D s_149_4: bits-cast zx s_149_3 -> bv length s_149_1
        let s_149_4: Bits = s_149_3.zero_extend(s_149_1);
        // D s_149_5: cast reint s_149_4 -> u64
        let s_149_5: u64 = (s_149_4.value() as u64);
        // D s_149_6: cast zx s_149_5 -> bv
        let s_149_6: Bits = Bits::new(s_149_5 as u128, 64u16);
        // D s_149_7: not s_149_6
        let s_149_7: Bits = !s_149_6;
        // D s_149_8: cast reint s_149_7 -> u64
        let s_149_8: u64 = (s_149_7.value() as u64);
        // D s_149_9: cast zx s_149_0 -> bv
        let s_149_9: Bits = Bits::new(s_149_0 as u128, 64u16);
        // D s_149_10: cast zx s_149_8 -> bv
        let s_149_10: Bits = Bits::new(s_149_8 as u128, 64u16);
        // D s_149_11: and s_149_9 s_149_10
        let s_149_11: Bits = ((s_149_9) & (s_149_10));
        // D s_149_12: cast reint s_149_11 -> u64
        let s_149_12: u64 = (s_149_11.value() as u64);
        // D s_149_13: call Mk_PMSNEVFR_EL1_Type(s_149_12)
        let s_149_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_149_12,
        );
        // D s_149_14: write-var tmp <= s_149_13
        fn_state.tmp = s_149_13;
        // N s_149_15: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_150_0: read-var tmp.0:struct
        let s_150_0: u64 = fn_state.tmp._0;
        // C s_150_1: const #64s : i
        let s_150_1: i128 = 64;
        // C s_150_2: const #32768u : u16
        let s_150_2: u16 = 32768;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 16u16);
        // D s_150_4: bits-cast zx s_150_3 -> bv length s_150_1
        let s_150_4: Bits = s_150_3.zero_extend(s_150_1);
        // D s_150_5: cast reint s_150_4 -> u64
        let s_150_5: u64 = (s_150_4.value() as u64);
        // D s_150_6: cast zx s_150_5 -> bv
        let s_150_6: Bits = Bits::new(s_150_5 as u128, 64u16);
        // D s_150_7: not s_150_6
        let s_150_7: Bits = !s_150_6;
        // D s_150_8: cast reint s_150_7 -> u64
        let s_150_8: u64 = (s_150_7.value() as u64);
        // D s_150_9: cast zx s_150_0 -> bv
        let s_150_9: Bits = Bits::new(s_150_0 as u128, 64u16);
        // D s_150_10: cast zx s_150_8 -> bv
        let s_150_10: Bits = Bits::new(s_150_8 as u128, 64u16);
        // D s_150_11: and s_150_9 s_150_10
        let s_150_11: Bits = ((s_150_9) & (s_150_10));
        // D s_150_12: cast reint s_150_11 -> u64
        let s_150_12: u64 = (s_150_11.value() as u64);
        // D s_150_13: call Mk_PMSNEVFR_EL1_Type(s_150_12)
        let s_150_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_150_12,
        );
        // D s_150_14: write-var tmp <= s_150_13
        fn_state.tmp = s_150_13;
        // N s_150_15: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_151_0: read-var tmp.0:struct
        let s_151_0: u64 = fn_state.tmp._0;
        // C s_151_1: const #64s : i
        let s_151_1: i128 = 64;
        // C s_151_2: const #16384u : u16
        let s_151_2: u16 = 16384;
        // C s_151_3: cast zx s_151_2 -> bv
        let s_151_3: Bits = Bits::new(s_151_2 as u128, 16u16);
        // D s_151_4: bits-cast zx s_151_3 -> bv length s_151_1
        let s_151_4: Bits = s_151_3.zero_extend(s_151_1);
        // D s_151_5: cast reint s_151_4 -> u64
        let s_151_5: u64 = (s_151_4.value() as u64);
        // D s_151_6: cast zx s_151_5 -> bv
        let s_151_6: Bits = Bits::new(s_151_5 as u128, 64u16);
        // D s_151_7: not s_151_6
        let s_151_7: Bits = !s_151_6;
        // D s_151_8: cast reint s_151_7 -> u64
        let s_151_8: u64 = (s_151_7.value() as u64);
        // D s_151_9: cast zx s_151_0 -> bv
        let s_151_9: Bits = Bits::new(s_151_0 as u128, 64u16);
        // D s_151_10: cast zx s_151_8 -> bv
        let s_151_10: Bits = Bits::new(s_151_8 as u128, 64u16);
        // D s_151_11: and s_151_9 s_151_10
        let s_151_11: Bits = ((s_151_9) & (s_151_10));
        // D s_151_12: cast reint s_151_11 -> u64
        let s_151_12: u64 = (s_151_11.value() as u64);
        // D s_151_13: call Mk_PMSNEVFR_EL1_Type(s_151_12)
        let s_151_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_151_12,
        );
        // D s_151_14: write-var tmp <= s_151_13
        fn_state.tmp = s_151_13;
        // N s_151_15: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_152_0: read-var tmp.0:struct
        let s_152_0: u64 = fn_state.tmp._0;
        // C s_152_1: const #64s : i
        let s_152_1: i128 = 64;
        // C s_152_2: const #8192u : u16
        let s_152_2: u16 = 8192;
        // C s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 16u16);
        // D s_152_4: bits-cast zx s_152_3 -> bv length s_152_1
        let s_152_4: Bits = s_152_3.zero_extend(s_152_1);
        // D s_152_5: cast reint s_152_4 -> u64
        let s_152_5: u64 = (s_152_4.value() as u64);
        // D s_152_6: cast zx s_152_5 -> bv
        let s_152_6: Bits = Bits::new(s_152_5 as u128, 64u16);
        // D s_152_7: not s_152_6
        let s_152_7: Bits = !s_152_6;
        // D s_152_8: cast reint s_152_7 -> u64
        let s_152_8: u64 = (s_152_7.value() as u64);
        // D s_152_9: cast zx s_152_0 -> bv
        let s_152_9: Bits = Bits::new(s_152_0 as u128, 64u16);
        // D s_152_10: cast zx s_152_8 -> bv
        let s_152_10: Bits = Bits::new(s_152_8 as u128, 64u16);
        // D s_152_11: and s_152_9 s_152_10
        let s_152_11: Bits = ((s_152_9) & (s_152_10));
        // D s_152_12: cast reint s_152_11 -> u64
        let s_152_12: u64 = (s_152_11.value() as u64);
        // D s_152_13: call Mk_PMSNEVFR_EL1_Type(s_152_12)
        let s_152_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_152_12,
        );
        // D s_152_14: write-var tmp <= s_152_13
        fn_state.tmp = s_152_13;
        // N s_152_15: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_153_0: read-var tmp.0:struct
        let s_153_0: u64 = fn_state.tmp._0;
        // C s_153_1: const #64s : i
        let s_153_1: i128 = 64;
        // C s_153_2: const #4096u : u16
        let s_153_2: u16 = 4096;
        // C s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 16u16);
        // D s_153_4: bits-cast zx s_153_3 -> bv length s_153_1
        let s_153_4: Bits = s_153_3.zero_extend(s_153_1);
        // D s_153_5: cast reint s_153_4 -> u64
        let s_153_5: u64 = (s_153_4.value() as u64);
        // D s_153_6: cast zx s_153_5 -> bv
        let s_153_6: Bits = Bits::new(s_153_5 as u128, 64u16);
        // D s_153_7: not s_153_6
        let s_153_7: Bits = !s_153_6;
        // D s_153_8: cast reint s_153_7 -> u64
        let s_153_8: u64 = (s_153_7.value() as u64);
        // D s_153_9: cast zx s_153_0 -> bv
        let s_153_9: Bits = Bits::new(s_153_0 as u128, 64u16);
        // D s_153_10: cast zx s_153_8 -> bv
        let s_153_10: Bits = Bits::new(s_153_8 as u128, 64u16);
        // D s_153_11: and s_153_9 s_153_10
        let s_153_11: Bits = ((s_153_9) & (s_153_10));
        // D s_153_12: cast reint s_153_11 -> u64
        let s_153_12: u64 = (s_153_11.value() as u64);
        // D s_153_13: call Mk_PMSNEVFR_EL1_Type(s_153_12)
        let s_153_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_153_12,
        );
        // D s_153_14: write-var tmp <= s_153_13
        fn_state.tmp = s_153_13;
        // N s_153_15: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_154_0: read-var tmp.0:struct
        let s_154_0: u64 = fn_state.tmp._0;
        // C s_154_1: const #64s : i
        let s_154_1: i128 = 64;
        // C s_154_2: const #2048u : u12
        let s_154_2: u16 = 2048;
        // C s_154_3: cast zx s_154_2 -> bv
        let s_154_3: Bits = Bits::new(s_154_2 as u128, 12u16);
        // D s_154_4: bits-cast zx s_154_3 -> bv length s_154_1
        let s_154_4: Bits = s_154_3.zero_extend(s_154_1);
        // D s_154_5: cast reint s_154_4 -> u64
        let s_154_5: u64 = (s_154_4.value() as u64);
        // D s_154_6: cast zx s_154_5 -> bv
        let s_154_6: Bits = Bits::new(s_154_5 as u128, 64u16);
        // D s_154_7: not s_154_6
        let s_154_7: Bits = !s_154_6;
        // D s_154_8: cast reint s_154_7 -> u64
        let s_154_8: u64 = (s_154_7.value() as u64);
        // D s_154_9: cast zx s_154_0 -> bv
        let s_154_9: Bits = Bits::new(s_154_0 as u128, 64u16);
        // D s_154_10: cast zx s_154_8 -> bv
        let s_154_10: Bits = Bits::new(s_154_8 as u128, 64u16);
        // D s_154_11: and s_154_9 s_154_10
        let s_154_11: Bits = ((s_154_9) & (s_154_10));
        // D s_154_12: cast reint s_154_11 -> u64
        let s_154_12: u64 = (s_154_11.value() as u64);
        // D s_154_13: call Mk_PMSNEVFR_EL1_Type(s_154_12)
        let s_154_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_154_12,
        );
        // D s_154_14: write-var tmp <= s_154_13
        fn_state.tmp = s_154_13;
        // N s_154_15: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_155_0: read-var tmp.0:struct
        let s_155_0: u64 = fn_state.tmp._0;
        // C s_155_1: const #64s : i
        let s_155_1: i128 = 64;
        // C s_155_2: const #1024u : u12
        let s_155_2: u16 = 1024;
        // C s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 12u16);
        // D s_155_4: bits-cast zx s_155_3 -> bv length s_155_1
        let s_155_4: Bits = s_155_3.zero_extend(s_155_1);
        // D s_155_5: cast reint s_155_4 -> u64
        let s_155_5: u64 = (s_155_4.value() as u64);
        // D s_155_6: cast zx s_155_5 -> bv
        let s_155_6: Bits = Bits::new(s_155_5 as u128, 64u16);
        // D s_155_7: not s_155_6
        let s_155_7: Bits = !s_155_6;
        // D s_155_8: cast reint s_155_7 -> u64
        let s_155_8: u64 = (s_155_7.value() as u64);
        // D s_155_9: cast zx s_155_0 -> bv
        let s_155_9: Bits = Bits::new(s_155_0 as u128, 64u16);
        // D s_155_10: cast zx s_155_8 -> bv
        let s_155_10: Bits = Bits::new(s_155_8 as u128, 64u16);
        // D s_155_11: and s_155_9 s_155_10
        let s_155_11: Bits = ((s_155_9) & (s_155_10));
        // D s_155_12: cast reint s_155_11 -> u64
        let s_155_12: u64 = (s_155_11.value() as u64);
        // D s_155_13: call Mk_PMSNEVFR_EL1_Type(s_155_12)
        let s_155_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_155_12,
        );
        // D s_155_14: write-var tmp <= s_155_13
        fn_state.tmp = s_155_13;
        // N s_155_15: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_156_0: const #"event 10 is implemented" : str
        let s_156_0: &'static str = "event 10 is implemented";
        // S s_156_1: call __IMPDEF_boolean(s_156_0)
        let s_156_1: bool = u__IMPDEF_boolean(state, tracer, s_156_0);
        // D s_156_2: write-var gs#37695 <= s_156_1
        fn_state.gs_37695 = s_156_1;
        // N s_156_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_157_0: const #1u : u8
        let s_157_0: bool = true;
        // D s_157_1: write-var gs#37694 <= s_157_0
        fn_state.gs_37694 = s_157_0;
        // N s_157_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_158_0: read-var tmp.0:struct
        let s_158_0: u64 = fn_state.tmp._0;
        // C s_158_1: const #64s : i
        let s_158_1: i128 = 64;
        // C s_158_2: const #512u : u12
        let s_158_2: u16 = 512;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 12u16);
        // D s_158_4: bits-cast zx s_158_3 -> bv length s_158_1
        let s_158_4: Bits = s_158_3.zero_extend(s_158_1);
        // D s_158_5: cast reint s_158_4 -> u64
        let s_158_5: u64 = (s_158_4.value() as u64);
        // D s_158_6: cast zx s_158_5 -> bv
        let s_158_6: Bits = Bits::new(s_158_5 as u128, 64u16);
        // D s_158_7: not s_158_6
        let s_158_7: Bits = !s_158_6;
        // D s_158_8: cast reint s_158_7 -> u64
        let s_158_8: u64 = (s_158_7.value() as u64);
        // D s_158_9: cast zx s_158_0 -> bv
        let s_158_9: Bits = Bits::new(s_158_0 as u128, 64u16);
        // D s_158_10: cast zx s_158_8 -> bv
        let s_158_10: Bits = Bits::new(s_158_8 as u128, 64u16);
        // D s_158_11: and s_158_9 s_158_10
        let s_158_11: Bits = ((s_158_9) & (s_158_10));
        // D s_158_12: cast reint s_158_11 -> u64
        let s_158_12: u64 = (s_158_11.value() as u64);
        // D s_158_13: call Mk_PMSNEVFR_EL1_Type(s_158_12)
        let s_158_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_158_12,
        );
        // D s_158_14: write-var tmp <= s_158_13
        fn_state.tmp = s_158_13;
        // N s_158_15: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_159_0: const #"event 9 is implemented" : str
        let s_159_0: &'static str = "event 9 is implemented";
        // S s_159_1: call __IMPDEF_boolean(s_159_0)
        let s_159_1: bool = u__IMPDEF_boolean(state, tracer, s_159_0);
        // D s_159_2: write-var gs#37693 <= s_159_1
        fn_state.gs_37693 = s_159_1;
        // N s_159_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_160_0: const #1u : u8
        let s_160_0: bool = true;
        // D s_160_1: write-var gs#37692 <= s_160_0
        fn_state.gs_37692 = s_160_0;
        // N s_160_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_161_0: read-var tmp.0:struct
        let s_161_0: u64 = fn_state.tmp._0;
        // C s_161_1: const #64s : i
        let s_161_1: i128 = 64;
        // C s_161_2: const #256u : u12
        let s_161_2: u16 = 256;
        // C s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 12u16);
        // D s_161_4: bits-cast zx s_161_3 -> bv length s_161_1
        let s_161_4: Bits = s_161_3.zero_extend(s_161_1);
        // D s_161_5: cast reint s_161_4 -> u64
        let s_161_5: u64 = (s_161_4.value() as u64);
        // D s_161_6: cast zx s_161_5 -> bv
        let s_161_6: Bits = Bits::new(s_161_5 as u128, 64u16);
        // D s_161_7: not s_161_6
        let s_161_7: Bits = !s_161_6;
        // D s_161_8: cast reint s_161_7 -> u64
        let s_161_8: u64 = (s_161_7.value() as u64);
        // D s_161_9: cast zx s_161_0 -> bv
        let s_161_9: Bits = Bits::new(s_161_0 as u128, 64u16);
        // D s_161_10: cast zx s_161_8 -> bv
        let s_161_10: Bits = Bits::new(s_161_8 as u128, 64u16);
        // D s_161_11: and s_161_9 s_161_10
        let s_161_11: Bits = ((s_161_9) & (s_161_10));
        // D s_161_12: cast reint s_161_11 -> u64
        let s_161_12: u64 = (s_161_11.value() as u64);
        // D s_161_13: call Mk_PMSNEVFR_EL1_Type(s_161_12)
        let s_161_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_161_12,
        );
        // D s_161_14: write-var tmp <= s_161_13
        fn_state.tmp = s_161_13;
        // N s_161_15: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_162_0: const #"event 8 is implemented" : str
        let s_162_0: &'static str = "event 8 is implemented";
        // S s_162_1: call __IMPDEF_boolean(s_162_0)
        let s_162_1: bool = u__IMPDEF_boolean(state, tracer, s_162_0);
        // D s_162_2: write-var gs#37691 <= s_162_1
        fn_state.gs_37691 = s_162_1;
        // N s_162_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_163_0: const #1u : u8
        let s_163_0: bool = true;
        // D s_163_1: write-var gs#37690 <= s_163_0
        fn_state.gs_37690 = s_163_0;
        // N s_163_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_164_0: read-var tmp.0:struct
        let s_164_0: u64 = fn_state.tmp._0;
        // C s_164_1: const #64s : i
        let s_164_1: i128 = 64;
        // C s_164_2: const #16u : u8
        let s_164_2: u8 = 16;
        // C s_164_3: cast zx s_164_2 -> bv
        let s_164_3: Bits = Bits::new(s_164_2 as u128, 8u16);
        // D s_164_4: bits-cast zx s_164_3 -> bv length s_164_1
        let s_164_4: Bits = s_164_3.zero_extend(s_164_1);
        // D s_164_5: cast reint s_164_4 -> u64
        let s_164_5: u64 = (s_164_4.value() as u64);
        // D s_164_6: cast zx s_164_5 -> bv
        let s_164_6: Bits = Bits::new(s_164_5 as u128, 64u16);
        // D s_164_7: not s_164_6
        let s_164_7: Bits = !s_164_6;
        // D s_164_8: cast reint s_164_7 -> u64
        let s_164_8: u64 = (s_164_7.value() as u64);
        // D s_164_9: cast zx s_164_0 -> bv
        let s_164_9: Bits = Bits::new(s_164_0 as u128, 64u16);
        // D s_164_10: cast zx s_164_8 -> bv
        let s_164_10: Bits = Bits::new(s_164_8 as u128, 64u16);
        // D s_164_11: and s_164_9 s_164_10
        let s_164_11: Bits = ((s_164_9) & (s_164_10));
        // D s_164_12: cast reint s_164_11 -> u64
        let s_164_12: u64 = (s_164_11.value() as u64);
        // D s_164_13: call Mk_PMSNEVFR_EL1_Type(s_164_12)
        let s_164_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_164_12,
        );
        // D s_164_14: write-var tmp <= s_164_13
        fn_state.tmp = s_164_13;
        // N s_164_15: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_165_0: const #1u : u8
        let s_165_0: bool = true;
        // D s_165_1: write-var gs#37689 <= s_165_0
        fn_state.gs_37689 = s_165_0;
        // N s_165_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_166_0: read-var tmp.0:struct
        let s_166_0: u64 = fn_state.tmp._0;
        // C s_166_1: const #64s : i
        let s_166_1: i128 = 64;
        // C s_166_2: const #4u : u8
        let s_166_2: u8 = 4;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 4u16);
        // D s_166_4: bits-cast zx s_166_3 -> bv length s_166_1
        let s_166_4: Bits = s_166_3.zero_extend(s_166_1);
        // D s_166_5: cast reint s_166_4 -> u64
        let s_166_5: u64 = (s_166_4.value() as u64);
        // D s_166_6: cast zx s_166_5 -> bv
        let s_166_6: Bits = Bits::new(s_166_5 as u128, 64u16);
        // D s_166_7: not s_166_6
        let s_166_7: Bits = !s_166_6;
        // D s_166_8: cast reint s_166_7 -> u64
        let s_166_8: u64 = (s_166_7.value() as u64);
        // D s_166_9: cast zx s_166_0 -> bv
        let s_166_9: Bits = Bits::new(s_166_0 as u128, 64u16);
        // D s_166_10: cast zx s_166_8 -> bv
        let s_166_10: Bits = Bits::new(s_166_8 as u128, 64u16);
        // D s_166_11: and s_166_9 s_166_10
        let s_166_11: Bits = ((s_166_9) & (s_166_10));
        // D s_166_12: cast reint s_166_11 -> u64
        let s_166_12: u64 = (s_166_11.value() as u64);
        // D s_166_13: call Mk_PMSNEVFR_EL1_Type(s_166_12)
        let s_166_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_166_12,
        );
        // D s_166_14: write-var tmp <= s_166_13
        fn_state.tmp = s_166_13;
        // N s_166_15: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_167_0: const #1u : u8
        let s_167_0: bool = true;
        // D s_167_1: write-var gs#37688 <= s_167_0
        fn_state.gs_37688 = s_167_0;
        // N s_167_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_168_0: read-var tmp.0:struct
        let s_168_0: u64 = fn_state.tmp._0;
        // C s_168_1: const #64s : i
        let s_168_1: i128 = 64;
        // C s_168_2: const #2u : u8
        let s_168_2: u8 = 2;
        // C s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 4u16);
        // D s_168_4: bits-cast zx s_168_3 -> bv length s_168_1
        let s_168_4: Bits = s_168_3.zero_extend(s_168_1);
        // D s_168_5: cast reint s_168_4 -> u64
        let s_168_5: u64 = (s_168_4.value() as u64);
        // D s_168_6: cast zx s_168_5 -> bv
        let s_168_6: Bits = Bits::new(s_168_5 as u128, 64u16);
        // D s_168_7: not s_168_6
        let s_168_7: Bits = !s_168_6;
        // D s_168_8: cast reint s_168_7 -> u64
        let s_168_8: u64 = (s_168_7.value() as u64);
        // D s_168_9: cast zx s_168_0 -> bv
        let s_168_9: Bits = Bits::new(s_168_0 as u128, 64u16);
        // D s_168_10: cast zx s_168_8 -> bv
        let s_168_10: Bits = Bits::new(s_168_8 as u128, 64u16);
        // D s_168_11: and s_168_9 s_168_10
        let s_168_11: Bits = ((s_168_9) & (s_168_10));
        // D s_168_12: cast reint s_168_11 -> u64
        let s_168_12: u64 = (s_168_11.value() as u64);
        // D s_168_13: call Mk_PMSNEVFR_EL1_Type(s_168_12)
        let s_168_13: ProductType5c790c8ef59cc8b2 = Mk_PMSNEVFR_EL1_Type(
            state,
            tracer,
            s_168_12,
        );
        // D s_168_14: write-var tmp <= s_168_13
        fn_state.tmp = s_168_13;
        // N s_168_15: jump b2
        return block_2(state, tracer, fn_state);
    }
}
