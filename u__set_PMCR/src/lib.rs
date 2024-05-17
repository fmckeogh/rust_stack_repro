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
use u_update_PMCR_Type_IMP::*;
use Mk_PMCR_Type::*;
use u_update_PMCR_Type_P::*;
use u_update_PMCR_Type_IDCODE::*;
use PMCR_read::*;
use PMCR_write::*;
use IsFeatureImplemented::*;
use u_get_PMCR_Type_IMP::*;
use u_get_PMCR_Type_P::*;
use u_get_PMCR_Type_C::*;
use u_update_PMCR_Type_C::*;
use u_get_PMCR_Type_IDCODE::*;
use common::*;
pub fn u__set_PMCR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        ga_26985: ProductType700c18a878c5601b,
        ga_26991: ProductType700c18a878c5601b,
        value_name: ProductType700c18a878c5601b,
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
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call PMCR_read(s_0_2)
        let s_0_3: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_0_2);
        // D s_0_4: write-var ga#26985 <= s_0_3
        fn_state.ga_26985 = s_0_3;
        // D s_0_5: read-var ga#26985.0:struct
        let s_0_5: u32 = fn_state.ga_26985._0;
        // D s_0_6: read-var r.0:struct
        let s_0_6: u32 = fn_state.r._0;
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // C s_0_8: const #1s : i
        let s_0_8: i128 = 1;
        // D s_0_9: cast zx s_0_6 -> bv
        let s_0_9: Bits = Bits::new(s_0_6 as u128, 32u16);
        // D s_0_10: bit-extract s_0_9 s_0_7 s_0_8
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_7)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: bool = ((s_0_10.value()) != 0);
        // C s_0_12: const #1s : i
        let s_0_12: i128 = 1;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // D s_0_14: cast zx s_0_5 -> bv
        let s_0_14: Bits = Bits::new(s_0_5 as u128, 32u16);
        // D s_0_15: cast zx s_0_11 -> bv
        let s_0_15: Bits = Bits::new(s_0_11 as u128, 1u16);
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: lsl s_0_17 s_0_12
        let s_0_18: Bits = s_0_17 << s_0_12;
        // C s_0_19: sub s_0_18 s_0_17
        let s_0_19: Bits = ((s_0_18) - (s_0_17));
        // D s_0_20: and s_0_15 s_0_19
        let s_0_20: Bits = ((s_0_15) & (s_0_19));
        // D s_0_21: lsl s_0_20 s_0_13
        let s_0_21: Bits = s_0_20 << s_0_13;
        // C s_0_22: lsl s_0_19 s_0_13
        let s_0_22: Bits = s_0_19 << s_0_13;
        // C s_0_23: cmpl s_0_22
        let s_0_23: Bits = !s_0_22;
        // D s_0_24: and s_0_14 s_0_23
        let s_0_24: Bits = ((s_0_14) & (s_0_23));
        // D s_0_25: or s_0_24 s_0_21
        let s_0_25: Bits = ((s_0_24) | (s_0_21));
        // D s_0_26: cast reint s_0_25 -> u32
        let s_0_26: u32 = (s_0_25.value() as u32);
        // D s_0_27: call Mk_PMCR_Type(s_0_26)
        let s_0_27: ProductType700c18a878c5601b = Mk_PMCR_Type(state, tracer, s_0_26);
        // D s_0_28: call PMCR_write(s_0_27)
        let s_0_28: () = PMCR_write(state, tracer, s_0_27);
        // C s_0_29: const #() : ()
        let s_0_29: () = ();
        // S s_0_30: call PMCR_read(s_0_29)
        let s_0_30: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_0_29);
        // D s_0_31: write-var ga#26991 <= s_0_30
        fn_state.ga_26991 = s_0_30;
        // D s_0_32: read-var ga#26991.0:struct
        let s_0_32: u32 = fn_state.ga_26991._0;
        // D s_0_33: read-var r.0:struct
        let s_0_33: u32 = fn_state.r._0;
        // C s_0_34: const #3s : i
        let s_0_34: i128 = 3;
        // C s_0_35: const #13s : i
        let s_0_35: i128 = 13;
        // D s_0_36: cast zx s_0_33 -> bv
        let s_0_36: Bits = Bits::new(s_0_33 as u128, 32u16);
        // D s_0_37: bit-extract s_0_36 s_0_34 s_0_35
        let s_0_37: Bits = (Bits::new(
            ((s_0_36) >> (s_0_34)).value(),
            u16::try_from(s_0_35).unwrap(),
        ));
        // D s_0_38: cast reint s_0_37 -> u13
        let s_0_38: u16 = (s_0_37.value() as u16);
        // C s_0_39: const #13s : i
        let s_0_39: i128 = 13;
        // C s_0_40: const #3s : i
        let s_0_40: i128 = 3;
        // D s_0_41: cast zx s_0_32 -> bv
        let s_0_41: Bits = Bits::new(s_0_32 as u128, 32u16);
        // D s_0_42: cast zx s_0_38 -> bv
        let s_0_42: Bits = Bits::new(s_0_38 as u128, 13u16);
        // C s_0_43: const #1u : u64
        let s_0_43: u64 = 1;
        // C s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 64u16);
        // C s_0_45: lsl s_0_44 s_0_39
        let s_0_45: Bits = s_0_44 << s_0_39;
        // C s_0_46: sub s_0_45 s_0_44
        let s_0_46: Bits = ((s_0_45) - (s_0_44));
        // D s_0_47: and s_0_42 s_0_46
        let s_0_47: Bits = ((s_0_42) & (s_0_46));
        // D s_0_48: lsl s_0_47 s_0_40
        let s_0_48: Bits = s_0_47 << s_0_40;
        // C s_0_49: lsl s_0_46 s_0_40
        let s_0_49: Bits = s_0_46 << s_0_40;
        // C s_0_50: cmpl s_0_49
        let s_0_50: Bits = !s_0_49;
        // D s_0_51: and s_0_41 s_0_50
        let s_0_51: Bits = ((s_0_41) & (s_0_50));
        // D s_0_52: or s_0_51 s_0_48
        let s_0_52: Bits = ((s_0_51) | (s_0_48));
        // D s_0_53: cast reint s_0_52 -> u32
        let s_0_53: u32 = (s_0_52.value() as u32);
        // D s_0_54: call Mk_PMCR_Type(s_0_53)
        let s_0_54: ProductType700c18a878c5601b = Mk_PMCR_Type(state, tracer, s_0_53);
        // D s_0_55: call PMCR_write(s_0_54)
        let s_0_55: () = PMCR_write(state, tracer, s_0_54);
        // C s_0_56: const #162u : u32
        let s_0_56: u32 = 162;
        // S s_0_57: call IsFeatureImplemented(s_0_56)
        let s_0_57: bool = IsFeatureImplemented(state, tracer, s_0_56);
        // S s_0_58: not s_0_57
        let s_0_58: bool = !s_0_57;
        // S s_0_59: not s_0_58
        let s_0_59: bool = !s_0_58;
        // N s_0_60: branch s_0_59 b6 b1
        if s_0_59 {
            return block_6(state, tracer, fn_state);
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call PMCR_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_2_0);
        // S s_2_2: call _get_PMCR_Type_IMP(s_2_1)
        let s_2_2: u8 = u_get_PMCR_Type_IMP(state, tracer, s_2_1);
        // S s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 8u16);
        // C s_2_4: const #0u : u8
        let s_2_4: u8 = 0;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 8u16);
        // S s_2_6: cmp-ne s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) != (s_2_5));
        // S s_2_7: not s_2_6
        let s_2_7: bool = !s_2_6;
        // N s_2_8: branch s_2_7 b5 b3
        if s_2_7 {
            return block_5(state, tracer, fn_state);
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
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call PMCR_read(s_4_0)
        let s_4_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_4_0);
        // D s_4_2: read-var r:struct
        let s_4_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_4_3: call _get_PMCR_Type_C(s_4_2)
        let s_4_3: bool = u_get_PMCR_Type_C(state, tracer, s_4_2);
        // D s_4_4: call _update_PMCR_Type_C(s_4_1, s_4_3)
        let s_4_4: ProductType700c18a878c5601b = u_update_PMCR_Type_C(
            state,
            tracer,
            s_4_1,
            s_4_3,
        );
        // D s_4_5: call PMCR_write(s_4_4)
        let s_4_5: () = PMCR_write(state, tracer, s_4_4);
        // C s_4_6: const #() : ()
        let s_4_6: () = ();
        // S s_4_7: call PMCR_read(s_4_6)
        let s_4_7: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_4_6);
        // D s_4_8: read-var r:struct
        let s_4_8: ProductType700c18a878c5601b = fn_state.r;
        // D s_4_9: call _get_PMCR_Type_P(s_4_8)
        let s_4_9: bool = u_get_PMCR_Type_P(state, tracer, s_4_8);
        // D s_4_10: call _update_PMCR_Type_P(s_4_7, s_4_9)
        let s_4_10: ProductType700c18a878c5601b = u_update_PMCR_Type_P(
            state,
            tracer,
            s_4_7,
            s_4_9,
        );
        // D s_4_11: call PMCR_write(s_4_10)
        let s_4_11: () = PMCR_write(state, tracer, s_4_10);
        // N s_4_12: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call PMCR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_5_0);
        // D s_5_2: read-var r:struct
        let s_5_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_5_3: call _get_PMCR_Type_IDCODE(s_5_2)
        let s_5_3: u8 = u_get_PMCR_Type_IDCODE(state, tracer, s_5_2);
        // D s_5_4: call _update_PMCR_Type_IDCODE(s_5_1, s_5_3)
        let s_5_4: ProductType700c18a878c5601b = u_update_PMCR_Type_IDCODE(
            state,
            tracer,
            s_5_1,
            s_5_3,
        );
        // D s_5_5: call PMCR_write(s_5_4)
        let s_5_5: () = PMCR_write(state, tracer, s_5_4);
        // N s_5_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call PMCR_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_6_0);
        // D s_6_2: read-var r:struct
        let s_6_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_6_3: call _get_PMCR_Type_IMP(s_6_2)
        let s_6_3: u8 = u_get_PMCR_Type_IMP(state, tracer, s_6_2);
        // D s_6_4: call _update_PMCR_Type_IMP(s_6_1, s_6_3)
        let s_6_4: ProductType700c18a878c5601b = u_update_PMCR_Type_IMP(
            state,
            tracer,
            s_6_1,
            s_6_3,
        );
        // D s_6_5: call PMCR_write(s_6_4)
        let s_6_5: () = PMCR_write(state, tracer, s_6_4);
        // N s_6_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
