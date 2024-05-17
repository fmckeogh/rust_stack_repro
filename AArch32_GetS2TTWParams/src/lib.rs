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
use u_get_VTCR_Type_T0SZ::*;
use u_get_VTCR_Type_IRGN0::*;
use u_get_HSCTLR_Type_EE::*;
use u_get_HCR_Type_PTW::*;
use u_get_VTCR_Type_S::*;
use HCR_read::*;
use u_get_VTCR_Type_SL0::*;
use u_get_VTCR_Type_ORGN0::*;
use u_get_HCR_Type_VM::*;
use u_get_VTCR_Type_SH0::*;
use u_get_HCR_Type_DC::*;
use ConstrainUnpredictableBits::*;
use VTCR_read::*;
use HSCTLR_read::*;
use common::*;
pub fn AArch32_GetS2TTWParams<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_27904: (),
) -> ProductTypeb05ce25a107f0c5e {
    #[derive(Default)]
    struct FunctionState {
        gs_453543: ProductType690b94b58c91cec7,
        walkparams: ProductTypeb05ce25a107f0c5e,
        gs_27904: (),
    }
    let fn_state = FunctionState {
        gs_27904,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: write-var walkparams.26 <= s_0_0
        fn_state.walkparams._26 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call VTCR_read(s_0_2)
        let s_0_3: ProductType700c18a878c5601b = VTCR_read(state, tracer, s_0_2);
        // S s_0_4: call _get_VTCR_Type_S(s_0_3)
        let s_0_4: bool = u_get_VTCR_Type_S(state, tracer, s_0_3);
        // D s_0_5: write-var walkparams.16 <= s_0_4
        fn_state.walkparams._16 = s_0_4;
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call VTCR_read(s_0_6)
        let s_0_7: ProductType700c18a878c5601b = VTCR_read(state, tracer, s_0_6);
        // S s_0_8: call _get_VTCR_Type_T0SZ(s_0_7)
        let s_0_8: u8 = u_get_VTCR_Type_T0SZ(state, tracer, s_0_7);
        // D s_0_9: write-var walkparams.25 <= s_0_8
        fn_state.walkparams._25 = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call VTCR_read(s_0_10)
        let s_0_11: ProductType700c18a878c5601b = VTCR_read(state, tracer, s_0_10);
        // S s_0_12: call _get_VTCR_Type_SL0(s_0_11)
        let s_0_12: u8 = u_get_VTCR_Type_SL0(state, tracer, s_0_11);
        // D s_0_13: write-var walkparams.22 <= s_0_12
        fn_state.walkparams._22 = s_0_12;
        // C s_0_14: const #() : ()
        let s_0_14: () = ();
        // S s_0_15: call VTCR_read(s_0_14)
        let s_0_15: ProductType700c18a878c5601b = VTCR_read(state, tracer, s_0_14);
        // S s_0_16: call _get_VTCR_Type_IRGN0(s_0_15)
        let s_0_16: u8 = u_get_VTCR_Type_IRGN0(state, tracer, s_0_15);
        // D s_0_17: write-var walkparams.10 <= s_0_16
        fn_state.walkparams._10 = s_0_16;
        // C s_0_18: const #() : ()
        let s_0_18: () = ();
        // S s_0_19: call VTCR_read(s_0_18)
        let s_0_19: ProductType700c18a878c5601b = VTCR_read(state, tracer, s_0_18);
        // S s_0_20: call _get_VTCR_Type_ORGN0(s_0_19)
        let s_0_20: u8 = u_get_VTCR_Type_ORGN0(state, tracer, s_0_19);
        // D s_0_21: write-var walkparams.13 <= s_0_20
        fn_state.walkparams._13 = s_0_20;
        // C s_0_22: const #() : ()
        let s_0_22: () = ();
        // S s_0_23: call VTCR_read(s_0_22)
        let s_0_23: ProductType700c18a878c5601b = VTCR_read(state, tracer, s_0_22);
        // S s_0_24: call _get_VTCR_Type_SH0(s_0_23)
        let s_0_24: u8 = u_get_VTCR_Type_SH0(state, tracer, s_0_23);
        // D s_0_25: write-var walkparams.20 <= s_0_24
        fn_state.walkparams._20 = s_0_24;
        // C s_0_26: const #() : ()
        let s_0_26: () = ();
        // S s_0_27: call HSCTLR_read(s_0_26)
        let s_0_27: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_0_26);
        // S s_0_28: call _get_HSCTLR_Type_EE(s_0_27)
        let s_0_28: bool = u_get_HSCTLR_Type_EE(state, tracer, s_0_27);
        // D s_0_29: write-var walkparams.4 <= s_0_28
        fn_state.walkparams._4 = s_0_28;
        // C s_0_30: const #() : ()
        let s_0_30: () = ();
        // S s_0_31: call HCR_read(s_0_30)
        let s_0_31: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_30);
        // S s_0_32: call _get_HCR_Type_PTW(s_0_31)
        let s_0_32: bool = u_get_HCR_Type_PTW(state, tracer, s_0_31);
        // D s_0_33: write-var walkparams.15 <= s_0_32
        fn_state.walkparams._15 = s_0_32;
        // C s_0_34: const #() : ()
        let s_0_34: () = ();
        // S s_0_35: call HCR_read(s_0_34)
        let s_0_35: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_34);
        // S s_0_36: call _get_HCR_Type_VM(s_0_35)
        let s_0_36: bool = u_get_HCR_Type_VM(state, tracer, s_0_35);
        // C s_0_37: const #() : ()
        let s_0_37: () = ();
        // S s_0_38: call HCR_read(s_0_37)
        let s_0_38: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_37);
        // S s_0_39: call _get_HCR_Type_DC(s_0_38)
        let s_0_39: bool = u_get_HCR_Type_DC(state, tracer, s_0_38);
        // S s_0_40: cast zx s_0_36 -> bv
        let s_0_40: Bits = Bits::new(s_0_36 as u128, 1u16);
        // S s_0_41: cast zx s_0_39 -> bv
        let s_0_41: Bits = Bits::new(s_0_39 as u128, 1u16);
        // S s_0_42: or s_0_40 s_0_41
        let s_0_42: Bits = ((s_0_40) | (s_0_41));
        // S s_0_43: cast reint s_0_42 -> u8
        let s_0_43: bool = ((s_0_42.value()) != 0);
        // D s_0_44: write-var walkparams.30 <= s_0_43
        fn_state.walkparams._30 = s_0_43;
        // D s_0_45: read-var walkparams.16:struct
        let s_0_45: bool = fn_state.walkparams._16;
        // D s_0_46: read-var walkparams.25:struct
        let s_0_46: u8 = fn_state.walkparams._25;
        // C s_0_47: const #3s : i
        let s_0_47: i128 = 3;
        // D s_0_48: cast zx s_0_46 -> bv
        let s_0_48: Bits = Bits::new(s_0_46 as u128, 4u16);
        // C s_0_49: const #1u : u64
        let s_0_49: u64 = 1;
        // D s_0_50: bit-extract s_0_48 s_0_47 s_0_49
        let s_0_50: Bits = (Bits::new(
            ((s_0_48) >> (s_0_47)).value(),
            u16::try_from(s_0_49).unwrap(),
        ));
        // D s_0_51: cast reint s_0_50 -> u8
        let s_0_51: bool = ((s_0_50.value()) != 0);
        // C s_0_52: const #0s : i
        let s_0_52: i128 = 0;
        // C s_0_53: const #0u : u64
        let s_0_53: u64 = 0;
        // D s_0_54: cast zx s_0_51 -> u64
        let s_0_54: u64 = (s_0_51 as u64);
        // C s_0_55: const #1u : u64
        let s_0_55: u64 = 1;
        // D s_0_56: and s_0_54 s_0_55
        let s_0_56: u64 = ((s_0_54) & (s_0_55));
        // D s_0_57: cmp-eq s_0_56 s_0_55
        let s_0_57: bool = ((s_0_56) == (s_0_55));
        // D s_0_58: lsl s_0_54 s_0_52
        let s_0_58: u64 = s_0_54 << s_0_52;
        // D s_0_59: or s_0_53 s_0_58
        let s_0_59: u64 = ((s_0_53) | (s_0_58));
        // D s_0_60: cmpl s_0_58
        let s_0_60: u64 = !s_0_58;
        // D s_0_61: and s_0_53 s_0_60
        let s_0_61: u64 = ((s_0_53) & (s_0_60));
        // D s_0_62: select s_0_57 s_0_59 s_0_61
        let s_0_62: u64 = if s_0_57 { s_0_59 } else { s_0_61 };
        // D s_0_63: cast trunc s_0_62 -> u8
        let s_0_63: bool = ((s_0_62) != 0);
        // D s_0_64: cast zx s_0_45 -> bv
        let s_0_64: Bits = Bits::new(s_0_45 as u128, 1u16);
        // D s_0_65: cast zx s_0_63 -> bv
        let s_0_65: Bits = Bits::new(s_0_63 as u128, 1u16);
        // D s_0_66: cmp-ne s_0_64 s_0_65
        let s_0_66: bool = ((s_0_64) != (s_0_65));
        // N s_0_67: branch s_0_66 b3 b1
        if s_0_66 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_2_0: read-var walkparams:struct
        let s_2_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_3_0: const #4s : i
        let s_3_0: i128 = 4;
        // C s_3_1: const #15u : u32
        let s_3_1: u32 = 15;
        // S s_3_2: call ConstrainUnpredictableBits(s_3_1, s_3_0)
        let s_3_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_3_1,
            s_3_0,
        );
        // D s_3_3: write-var gs#453543 <= s_3_2
        fn_state.gs_453543 = s_3_2;
        // D s_3_4: read-var gs#453543.1:struct
        let s_3_4: Bits = fn_state.gs_453543._1;
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: u8 = (s_3_4.value() as u8);
        // D s_3_6: write-var walkparams.25 <= s_3_5
        fn_state.walkparams._25 = s_3_5;
        // N s_3_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
